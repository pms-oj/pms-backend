use crate::constants::*;
use actix::prelude::*;
use ckydb::{connect, controller::Ckydb, Controller};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug, MessageResponse)]
pub enum KeyDbResponse {
    None,
    Data(String),
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "KeyDbResponse")]
// CRUD operation
pub enum KeyDbMessage {
    Insert(Uuid, String),
    Get(Uuid),
}

pub struct KeyDbService {
    pub raw: Ckydb,
    pub path: String,
}

impl KeyDbService {
    pub fn start(path: &str, threads: usize) -> Addr<Self> {
        let pathc = path.to_string();
        SyncArbiter::start(threads, move || Self {
            path: pathc.clone(),
            raw: connect(pathc.as_str(), MAX_FILE_SIZE_KB, VACUUM_INTERVAL_SEC).unwrap(),
        })
    }
}

impl Actor for KeyDbService {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!(
            "Started key-based database service in {}",
            self.path.clone()
        );
    }
}

impl Handler<KeyDbMessage> for KeyDbService {
    type Result = KeyDbResponse;

    fn handle(&mut self, msg: KeyDbMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            KeyDbMessage::Get(uuid) => {
                if let Ok(val) = self.raw.get(&uuid.to_string()) {
                    KeyDbResponse::Data(val)
                } else {
                    KeyDbResponse::None
                }
            }
            KeyDbMessage::Insert(uuid, data) => {
                self.raw.set(&uuid.to_string(), &data).ok();
                KeyDbResponse::None
            }
        }
    }
}
