use async_graphql::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, Enum, Copy, PartialEq, Eq, Default)]
pub enum TestState {
    #[default]
    Ready,
    Pending,
    Success,
    RuntimeErr,
    DiedOnSignal,
    TimeLimitExceed,
    MemLimitExceed,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize, Enum, Copy, PartialEq, Eq, Default)]
pub enum SubmissionState {
    #[default]
    Ready,
    DoCompile,
    CompileComplete,
    CompileError,
    Pending,
    Failed,
    Success,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Submission {
    pub judge_uuid: Uuid,
    pub submission_state: SubmissionState,
    pub number_of_cases: usize,
    pub number_of_done: usize,
    pub compile_output: String,
    pub score: f64,
    pub cases: HashMap<Uuid, TestCase>,
}

impl Submission {
    pub fn to_json(&self) -> String {
        simd_json::to_string(&self).unwrap()
    }

    pub fn from_json(raw: String) -> Self {
        simd_json::serde::from_borrowed_value(raw.into()).unwrap()
    }
}

#[Object]
impl Submission {
    async fn uuid(&self) -> Uuid {
        self.judge_uuid
    }

    async fn number_of_cases(&self) -> usize {
        self.number_of_cases
    }

    async fn number_of_done(&self) -> usize {
        self.number_of_done
    }

    async fn compile_output(&self) -> String {
        self.compile_output.clone()
    }

    async fn score(&self) -> f64 {
        self.score
    }

    async fn cases(&self) -> HashMap<Uuid, TestCase> {
        self.cases.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub test_uuid: Uuid,
    pub status: TestState,
    pub score: Option<f64>,
    pub exit_code: Option<i32>,
    pub exit_sig: Option<i32>,
    pub time: Option<u64>, // in ms
    pub mem: Option<u64>,  // in kB
}

#[Object]
impl TestCase {
    async fn uuid(&self) -> Uuid {
        self.test_uuid
    }

    async fn status(&self) -> TestState {
        self.status
    }

    async fn score(&self) -> Option<f64> {
        self.score
    }

    async fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    async fn exit_sig(&self) -> Option<i32> {
        self.exit_sig
    }

    async fn time(&self) -> Option<u64> {
        self.time
    }

    async fn mem(&self) -> Option<u64> {
        self.mem
    }
}
