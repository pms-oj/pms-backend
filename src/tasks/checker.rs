use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checker {
    pub checker_language: Uuid,
    pub checker_file: String,
}