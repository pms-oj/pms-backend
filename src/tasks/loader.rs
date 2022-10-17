use async_std::fs::*;
use async_std::io;
use async_std::path::PathBuf;
use async_std::stream::StreamExt;
use lcid::LanguageId;
use std::collections::HashMap;
use std::convert::TryInto;
use uuid::Uuid;

use super::checker::*;
use super::constants::*;
use super::graders::*;
use super::statements::*;
use super::subtasks::*;
use super::*;
use crate::judge::*;

#[derive(Clone, Debug)]
pub struct TaskLoader {
    pub task: Task,
    pub subtasks: SubtaskLoader,
    pub graders: GraderLoader,
    pub statements: StatementLoader,
    pub checker: CheckerLoader,
    pub attachments: PathBuf,
}

#[derive(Clone, Debug)]
pub struct SubtaskLoader {
    pub subtasks: HashMap<String, (Subtask, Vec<Test>)>,
}

#[derive(Clone, Debug)]
pub struct StatementLoader {
    pub statements: HashMap<LanguageId, (Statement, PathBuf)>,
}

#[derive(Clone, Debug)]
pub struct GraderLoader {
    pub grader: Grader,
    pub manager_file: Option<PathBuf>,
    pub graders: HashMap<Uuid, PathBuf>,
}

#[derive(Clone, Debug)]
pub struct CheckerLoader {
    pub checker: Checker,
    pub checker_file: PathBuf,
}

pub async fn load_task(path: PathBuf) -> io::Result<TaskLoader> {
    assert_eq!(path.is_dir().await, true);
    let mut task = None;
    let mut entries = read_dir(path).await?;
    let mut attachments = None;
    let mut grader_loader = None;
    let mut statement_loader = None;
    let mut checker_loader = None;
    let mut subtask_loader = None;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        if let Ok(file_t) = entry.file_type().await {
            if file_t.is_file() {
                if entry.file_name().to_str().unwrap() == TASK_TOML {
                    let f = read_to_string(entry.path().clone()).await?;
                    task = Some(toml::from_str(&f).expect("Cannot read task.toml"));
                }
            } else {
                match entry.file_name().to_str().unwrap() {
                    CHECKER => {
                        checker_loader = Some(load_checker(entry.path()).await?);
                    }
                    SUBTASKS => subtask_loader = Some(load_subtasks(entry.path()).await?),
                    GRADERS => {
                        grader_loader = Some(load_graders(entry.path()).await?);
                    }
                    STATEMENTS => {
                        statement_loader = Some(load_statements(entry.path()).await?);
                    }
                    ATTACHMENTS => {
                        attachments = Some(entry.path().clone());
                    }
                    _ => {}
                }
            }
        }
    }
    debug!("{:?}", task.clone());
    if let (
        Some(task),
        Some(checker),
        Some(subtasks),
        Some(graders),
        Some(statements),
        Some(attachments),
    ) = (
        task,
        checker_loader,
        subtask_loader,
        grader_loader,
        statement_loader,
        attachments,
    ) {
        Ok(TaskLoader {
            task,
            checker,
            subtasks,
            graders,
            statements,
            attachments,
        })
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

pub async fn load_statements(path: PathBuf) -> io::Result<StatementLoader> {
    assert_eq!(path.is_dir().await, true);
    let mut statements = HashMap::new();
    let mut entries = read_dir(path.clone()).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        if let Ok(file_t) = entry.file_type().await {
            if file_t.is_dir() {
                let fname = entry.file_name();
                let name = fname.to_str().unwrap();
                if let Ok(lang) = TryInto::<&LanguageId>::try_into(name.clone()) {
                    let statement_path = entry.path().join(STATEMENT_TOML);
                    let f = read_to_string(statement_path).await?;
                    let statement: Statement =
                        toml::from_str(&f).expect("Cannot read statement.toml");
                    statements.insert(lang.clone(), (statement, entry.path()));
                }
            }
        }
    }
    Ok(StatementLoader { statements })
}

pub async fn load_subtasks(path: PathBuf) -> io::Result<SubtaskLoader> {
    assert_eq!(path.is_dir().await, true);
    let mut subtasks = HashMap::new();
    let mut entries = read_dir(path.clone()).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        if let Ok(file_t) = entry.file_type().await {
            if file_t.is_file() && entry.path().extension().unwrap() == "toml" {
                let f = read_to_string(entry.path()).await?;
                let subtask: Subtask = toml::from_str(&f).expect("Cannot read {subtask name}.toml");
                let mut tests = vec![];
                for x in subtask.testcases.clone() {
                    tests.push(Test {
                        stdin: path
                            .parent()
                            .unwrap()
                            .join(TESTS)
                            .join(x.clone())
                            .with_extension("in"),
                        stdout: path
                            .parent()
                            .unwrap()
                            .join(TESTS)
                            .join(x.clone())
                            .with_extension("out"),
                    });
                }
                subtasks.insert(subtask.name.clone(), (subtask, tests));
            }
        }
    }
    Ok(SubtaskLoader { subtasks })
}

pub async fn load_checker(path: PathBuf) -> io::Result<CheckerLoader> {
    assert_eq!(path.is_dir().await, true);
    let mut checker: Option<Checker> = None;
    let mut entries = read_dir(path.clone()).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        if let Ok(file_t) = entry.file_type().await {
            if file_t.is_file() {
                match entry.file_name().to_str().unwrap() {
                    CHECKER_TOML => {
                        let f = read_to_string(entry.path()).await?;
                        checker = Some(toml::from_str(&f).expect("Cannot read checker.toml"));
                    }
                    _ => {}
                }
            }
        }
    }
    if let Some(checker) = checker {
        let checker_path = path.join(checker.checker_file.clone());
        if checker_path.is_file().await {
            Ok(CheckerLoader {
                checker,
                checker_file: checker_path,
            })
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound))
        }
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

pub async fn load_graders(path: PathBuf) -> io::Result<GraderLoader> {
    assert_eq!(path.is_dir().await, true);
    let mut graders = HashMap::new();
    let mut entries = read_dir(path.clone()).await?;
    let mut grader: Option<Grader> = None;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        if let Ok(file_t) = entry.file_type().await {
            if file_t.is_file() {
                match entry.file_name().to_str().unwrap() {
                    GRADER_TOML => {
                        let f = read_to_string(entry.path()).await?;
                        grader = Some(toml::from_str(&f).expect("Cannot read grader.toml"));
                    }
                    _ => {}
                }
            } else {
                let fname = entry.file_name();
                let dir_name = fname.to_str().unwrap();
                if let Ok(lang_uuid) = dir_name.parse::<Uuid>() {
                    graders.insert(lang_uuid, entry.path());
                }
            }
        }
    }
    if let Some(grader) = grader {
        if let Some(manager) = grader.manager_file.clone() {
            let manager_path = path.join(manager);
            if manager_path.is_file().await {
                Ok(GraderLoader {
                    grader,
                    manager_file: Some(manager_path),
                    graders,
                })
            } else {
                Err(io::Error::from(io::ErrorKind::NotFound))
            }
        } else {
            Ok(GraderLoader {
                grader,
                manager_file: None,
                graders,
            })
        }
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}
