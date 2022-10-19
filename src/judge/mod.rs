use actix::prelude::*;
use async_std::channel::{unbounded, Receiver, Sender};
use async_std::path::PathBuf;
use async_std::task::spawn;
use serde::{Deserialize, Serialize};

use pms_master::event::*;
use pms_master::handler::*;
use pms_master::judge::*;

use judge_protocol::judge::*;

use uuid::Uuid;

use futures_util::{Stream, StreamExt, TryFutureExt};

use std::collections::HashMap;

use crate::db::keydb::*;
use crate::judge::api::*;
use crate::APPDATA;

pub mod api;

#[derive(Clone, Debug)]
pub struct Test {
    pub stdin: PathBuf,
    pub stdout: PathBuf,
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub enum JudgeMessage {
    Enqueue(RequestJudge),
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "StateStream")]
pub enum SubscribeMessage {
    Subscribe(Uuid),
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub enum DropMessage {
    Drop(Uuid, usize),
}

#[derive(MessageResponse)]
pub struct StateStream(Uuid, Addr<JudgeService>, usize, Receiver<Submission>);

impl Drop for StateStream {
    fn drop(&mut self) {
        self.1.do_send(DropMessage::Drop(self.0, self.2));
    }
}

impl Stream for StateStream {
    type Item = Submission;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.3.poll_next_unpin(cx)
    }
}

pub struct JudgeService {
    pub judge_addrs: HashMap<Uuid, Vec<Option<Sender<Submission>>>>,
}

impl Actor for JudgeService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        info!("Started pms-backend judge service");
    }
}

impl Handler<SubscribeMessage> for JudgeService {
    type Result = StateStream;

    fn handle(&mut self, msg: SubscribeMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SubscribeMessage::Subscribe(uuid) => {
                if let None = self.judge_addrs.get_mut(&uuid) {
                    self.judge_addrs.insert(uuid, vec![]);
                }
                let (tx, rx) = unbounded();
                let sz = self.judge_addrs[&uuid].len();
                if let Some(judge_addrs) = self.judge_addrs.get_mut(&uuid) {
                    judge_addrs.push(Some(tx.clone()));
                }
                StateStream(uuid, ctx.address(), sz, rx)
            }
        }
    }
}

impl Handler<DropMessage> for JudgeService {
    type Result = ();

    fn handle(&mut self, msg: DropMessage, ctx: &mut Self::Context) -> Self::Result {
        if let DropMessage::Drop(uuid, idx) = msg {
            if let Some(v) = self.judge_addrs.get_mut(&uuid) {
                v[idx] = None;
            }
        }
    }
}

impl Handler<EventMessage> for JudgeService {
    type Result = ();

    fn handle(&mut self, msg: EventMessage, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            EventMessage::JudgeResult(judge_uuid, state) => {
                let set = self.judge_addrs.get(&judge_uuid).unwrap().clone();
                let sys = System::new();
                sys.block_on(async move {
                    if let KeyDbResponse::Data(data) = APPDATA
                        .judge_db
                        .send(KeyDbMessage::Get(judge_uuid))
                        .await
                        .unwrap()
                    {
                        let mut cur_state = Submission::from_json(data);
                        match state {
                            JudgeState::CompleteCompile(out) => {
                                cur_state.submission_state = SubmissionState::CompileComplete;
                                cur_state.compile_output = out;
                            }
                            JudgeState::UnknownError
                            | JudgeState::JudgeNotFound
                            | JudgeState::LanguageNotFound
                            | JudgeState::LockedSlave => {
                                // EOJ
                                cur_state.submission_state = SubmissionState::Failed;
                            }
                            JudgeState::DiedOnSignal(test_uuid, exit_sig) => {
                                // EOJ for Test
                                if let Some(data) = cur_state.cases.get_mut(&test_uuid) {
                                    data.exit_sig = Some(exit_sig);
                                    data.status = TestState::DiedOnSignal;
                                } else {
                                    panic!("Judgement database is corrupted")
                                }
                                cur_state.number_of_done += 1;
                            }
                            JudgeState::Accepted(test_uuid, time, mem) => {
                                // EOJ for Test
                                // TODO: score
                                if let Some(data) = cur_state.cases.get_mut(&test_uuid) {
                                    data.time = Some(time);
                                    data.mem = Some(mem);
                                    data.status = TestState::Success;
                                    data.score = Some(1.0);
                                } else {
                                    panic!("Judgement database is corrupted")
                                }
                                cur_state.score += 1.0;
                                cur_state.number_of_done += 1;
                            }
                            JudgeState::WrongAnswer(test_uuid, time, mem) => {
                                // EOJ for Test
                                // TODO: score
                                if let Some(data) = cur_state.cases.get_mut(&test_uuid) {
                                    data.time = Some(time);
                                    data.mem = Some(mem);
                                    data.status = TestState::Success;
                                } else {
                                    panic!("Judgement database is corrupted")
                                }
                                cur_state.number_of_done += 1;
                            }
                            JudgeState::GeneralError(out) => {
                                error!(
                                    "Judgement service has received JudgeState::GeneralError: {}",
                                    out
                                );
                                cur_state.submission_state = SubmissionState::Failed;
                            }
                            JudgeState::InternalError(test_uuid) => {
                                // EOJ for Test
                                if let Some(data) = cur_state.cases.get_mut(&test_uuid) {
                                    data.status = TestState::Failed;
                                } else {
                                    panic!("Judgement database is corrupted")
                                }
                                cur_state.number_of_done += 1;
                            }
                            JudgeState::TimeLimitExceed(test_uuid) => {
                                // EOJ for Test
                                if let Some(data) = cur_state.cases.get_mut(&test_uuid) {
                                    data.status = TestState::TimeLimitExceed;
                                } else {
                                    panic!("Judgement database is corrupted")
                                }
                                cur_state.number_of_done += 1;
                            }
                            JudgeState::MemLimitExceed(test_uuid) => {
                                // EOJ for Test
                                if let Some(data) = cur_state.cases.get_mut(&test_uuid) {
                                    data.status = TestState::MemLimitExceed;
                                } else {
                                    panic!("Judgement database is corrupted")
                                }
                                cur_state.number_of_done += 1;
                            }
                            JudgeState::RuntimeError(test_uuid, exit_code) => {
                                // EOJ for Test
                                if let Some(data) = cur_state.cases.get_mut(&test_uuid) {
                                    data.exit_code = Some(exit_code);
                                    data.status = TestState::RuntimeErr;
                                } else {
                                    panic!("Judgement database is corrupted")
                                }
                                cur_state.number_of_done += 1;
                            }
                            JudgeState::CompileError(stderr) => {
                                // EOJ
                                cur_state.submission_state = SubmissionState::CompileError;
                                cur_state.compile_output = stderr;
                            }
                            JudgeState::DoCompile => {
                                cur_state.submission_state = SubmissionState::DoCompile;
                            }
                            _ => {}
                        }
                        for tx in set {
                            if let Some(tx) = tx {
                                let s = cur_state.clone();
                                spawn(async move { tx.send(s).await });
                            }
                        }
                        let submission = cur_state.to_json();
                        APPDATA
                            .judge_db
                            .send(KeyDbMessage::Insert(judge_uuid, submission))
                            .await
                            .ok();
                    }
                });
            }
            _ => {}
        }
    }
}

impl Handler<JudgeMessage> for JudgeService {
    type Result = ();

    fn handle(&mut self, msg: JudgeMessage, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            JudgeMessage::Enqueue(judge) => {
                APPDATA
                    .state
                    .handler_addr
                    .do_send(HandlerMessage::Judge(judge));
            }
        }
    }
}
