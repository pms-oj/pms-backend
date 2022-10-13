use serde::{Serialize, Deserialize};
use uuid::Uuid;

// * note that for some notations
// [x]: enumeration of arbitrary elements which are following rule x
// {x}: indefinite variable x
// *: optional; not required
// Directory structure of PMS task v1
//  ../graders
//  - grader.toml
//  - {manager file}
//  - * [{language UUID}]
//      - stub.toml
//      - Makefile
//      - [{stub file}]

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grader {
    pub manager_file: Option<String>,
    pub manager_language: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stub {
    pub object_file: String, 
}