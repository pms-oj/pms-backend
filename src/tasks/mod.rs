pub mod checker;
pub mod constants;
pub mod graders;
pub mod loader;
pub mod statements;
pub mod subtasks;

pub use loader::*;

use serde::{Deserialize, Serialize};

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
