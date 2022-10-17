use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checker {
    pub checker_language: Uuid,
    pub checker_file: String,
}
