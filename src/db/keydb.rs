use actix::prelude::*;
use ckydb::{connect, Controller};
use std::pin::Pin;
use uuid::Uuid;

#[derive(Clone, Debug, MessageResponse)]
pub enum KeyDbResponse {
    None,
    Data(Vec<u8>),
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "KeyDbResponse")]
// CRUD operation
pub enum KeyDbMessage {
    Insert(Uuid, Vec<u8>),
    Get(Uuid),
}

pub struct KeyDbService<T>
where
    T: Controller + 'static + Unpin,
{
    pub raw: T,
}

impl<T> KeyDbService<T>
where
    T: Controller + 'static + Unpin,
{
    pub fn start(raw_db: T) -> Addr<Self> {
        Self { raw: raw_db }.start()
    }
}

impl<T> Actor for KeyDbService<T>
where
    T: Controller + 'static + Unpin,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Started key-based database service");
    }
}

impl<T> Handler<KeyDbMessage> for KeyDbService<T>
where
    T: Controller + 'static + Unpin,
{
    type Result = KeyDbResponse;

    fn handle(&mut self, msg: KeyDbMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            KeyDbMessage::Get(uuid) => {
                if let Ok(val) = self.raw.get(&uuid.to_string()) {
                    KeyDbResponse::Data(val.as_bytes().to_vec())
                } else {
                    KeyDbResponse::None
                }
            }
            KeyDbMessage::Insert(uuid, data) => {
                self.raw
                    .set(&uuid.to_string(), &String::from_utf8(data).unwrap())
                    .ok();
                KeyDbResponse::None
            }
        }
    }
}
