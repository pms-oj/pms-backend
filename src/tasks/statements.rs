use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StatementFormat {
    Markdown,
    Tex,
    Pdf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Statement {
    pub input: StatementFormat,
    pub output: StatementFormat,
    pub legend: StatementFormat,
    pub name: StatementFormat,
    pub notes: StatementFormat,
}