pub mod checker;
pub mod constants;
pub mod graders;
pub mod loader;
pub mod statements;
pub mod subtasks;

pub use loader::*;

use actix::prelude::*;
use async_std::fs::File;
use async_std::io;
use async_std::path::PathBuf;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::constants::*;
use crate::middlewares::postgresql::*;

// implementation of https://cms.readthedocs.io/en/v1.4/Task%20types.html
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskTypes {
    Batch,
    OutputOnly,
    Communication,
    TwoSteps,
    Custom(()), // not implemented yet
}

// * note that for some notations
// [x]: enumeration of arbitrary elements which are following rule x
// {x}: indefinite variable x
// *: optional; not required
// Directory structure of PMS task v1
//  ../{task UUID}
//  - task.toml
//  - checker
//      - checker.toml
//      - {checker file}
//  - tests
//      - [{standard input}.in]
//      - [{standard output}.out]
//  - subtasks
//      - [{name of subtask}.toml]
//  - statements
//      - [{IETF language tag}]
//          - statement.toml
//          - [statement.{tex, pdf, md}]
//  - graders
//      - grader.toml
//      - {manager file}
//      - * [{language UUID}]
//          - stub.toml
//          - Makefile
//          - [{stub file}]
//  - attachments
//      - [{attachment file}]

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub code: String,
    pub task_type: TaskTypes,
    pub time_limit: f64,
    pub memory_limit: u64,
    pub score_precision: Option<usize>,
    pub task_type_params: Parms,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Parms {
    pub num_processes: Option<usize>, // for default, num_processes = 1
}

#[derive(Clone, Debug, MessageResponse)]
pub enum TasksResponse {
    String(String),
    Usize(usize),
    Meta(Task),
    None,
    Error,
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "TasksResponse")]
pub enum TasksMessage {
    GetMeta(Uuid),
    GetName(Uuid),
    GetInput(Uuid),
    GetOutput(Uuid),
    GetLegend(Uuid),
    GetNotes(Uuid),
}

pub struct TasksService {
    db: PgConnection,
    cache: HashMap<Uuid, TaskLoader>,
}

impl TasksService {
    fn start(threads: usize) -> Addr<TasksService> {
        SyncArbiter::start(threads, || Self {
            db: establish_connection(),
            cache: HashMap::new(),
        })
    }

    async fn read_to_cache(&mut self, uuid: Uuid) -> io::Result<()> {
        self.cache.insert(
            uuid,
            load_task(PathBuf::from(TASKS).join(uuid.to_string())).await?,
        );
        Ok(())
    }
}

// Maybe only for usage on RO(Read-Only)
impl Actor for TasksService {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("Started tasks service");
    }
}

impl Handler<TasksMessage> for TasksService {
    type Result = TasksResponse;

    fn handle(&mut self, msg: TasksMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            TasksMessage::GetMeta(uuid) => {
                let sys = System::new();
                sys.block_on(async move {
                    if let Ok(_) = self.read_to_cache(uuid).await {
                        if let Some(x) = self.cache.get(&uuid) {
                            TasksResponse::Meta(x.task.clone())
                        } else {
                            TasksResponse::Error
                        }
                    } else {
                        TasksResponse::Error
                    }
                })
            }
            _ => TasksResponse::None,
        }
    }
}
