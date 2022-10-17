use actix::prelude::*;
use async_std::path::PathBuf;

use pms_master::event::*;
use pms_master::handler::*;
use pms_master::judge::*;

use judge_protocol::judge::*;

use uuid::Uuid;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Test {
    pub stdin: PathBuf,
    pub stdout: PathBuf,
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub enum JudgeMessage {
    RegisterHandler(Addr<HandlerService<JudgeService>>),
    Enqueue(RequestJudge),
}

#[derive(Clone, Debug)]
pub struct JudgeMan {
    pub judge_addrs: HashMap<Uuid, Addr<StateService>>,
}

pub struct StateService {
    pub uuid: Uuid,
}

impl Actor for StateService {
    type Context = Context<Self>;
}

impl Handler<JudgeState> for StateService {
    type Result = ();

    fn handle(&mut self, msg: JudgeState, ctx: &mut Context<Self>) -> Self::Result {}
}

pub struct JudgeService {
    pub judge_man: JudgeMan,
    pub handler_addr: Option<Addr<HandlerService<Self>>>,
}

impl Actor for JudgeService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        info!("Started pms-backend judge service");
    }
}

impl Handler<EventMessage> for JudgeService {
    type Result = ();

    fn handle(&mut self, msg: EventMessage, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            EventMessage::JudgeResult(judge_uuid, state) => {
                if let Some(addr) = self.judge_man.judge_addrs.get(&judge_uuid) {
                    addr.do_send(state);
                } else {
                    error!("Can't get judge state actor addr for {}", judge_uuid);
                }
            }
            _ => {}
        }
    }
}

impl Handler<JudgeMessage> for JudgeService {
    type Result = ();

    fn handle(&mut self, msg: JudgeMessage, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            JudgeMessage::RegisterHandler(addr) => {
                self.handler_addr = Some(addr);
                info!("Registered handler service to judge service");
            }
            JudgeMessage::Enqueue(judge) => {
                if let Some(handler_addr) = &self.handler_addr {
                    handler_addr.do_send(HandlerMessage::Judge(judge));
                } else {
                    error!("Unable to get handler address");
                }
            }
        }
    }
}
