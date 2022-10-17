use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subtask {
    pub name: String,
    pub score: f64,
    pub testcases: Vec<String>,
}
