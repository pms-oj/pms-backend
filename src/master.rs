use async_std::channel::{Receiver, Sender};
use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};
use pms_master::event::EventMessage;
use pms_master::handler::HandlerMessage;

use crate::JudgeMan;

pub async fn master_handler(
    judge_man: Arc<Mutex<JudgeMan>>,
    handler_tx: Sender<HandlerMessage>,
    mut event_rx: Receiver<EventMessage>,
) {
    while let Some(msg) = event_rx.next().await {
        match msg {
            EventMessage::JudgeResult(judge_uuid, state) => {
                if let Some(tx) = judge_man.lock().await.judge_tx.get(&judge_uuid) {
                    tx.send(state).await.ok();
                } else {
                    error!("Can't get judge tx for {}", judge_uuid);
                }
            }
            _ => {}
        }
    }
}
