#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod api;
mod config;
mod constants;
mod db;
mod master;
mod middlewares;
mod tasks;
mod contests;
mod judge;

#[cfg(test)]
mod tests;

use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use async_std::channel::{unbounded, Sender};
use async_std::sync::{Arc, Mutex};
use async_std::task::spawn;
use ckydb::{connect, Controller};
use judge_protocol::judge::*;
use log::*;
use pms_master::handler::{serve, HandlerMessage};
use std::collections::HashMap;
use std::fs::read_to_string;
use uuid::Uuid;

use crate::config::*;
use crate::constants::*;
use crate::master::master_handler;

lazy_static! {
    static ref CONFIG: Config = {
        let s = read_to_string(CONFIG_FILE).expect("Some error occured");
        info!("Loaded PMS backend config file");
        toml::from_str(&s).expect("Some error occured")
    };
}

#[derive(Clone)]
pub struct WebState {
    handler_tx: Sender<HandlerMessage>,
}

#[derive(Clone, Debug)]
pub struct JudgeMan {
    judge_tx: HashMap<Uuid, Sender<JudgeState>>,
}

#[derive(Clone)]
pub struct WebData<T: Controller> {
    state: WebState,
    judge_man: Arc<Mutex<JudgeMan>>,
    judge_db: Arc<Mutex<T>>,
    source_db: Arc<Mutex<T>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file(LOG_CONFIG_FILE, Default::default()).unwrap();
    info!("pms-backend {}", env!("CARGO_PKG_VERSION"));
    let judge_db = connect(JUDGE_DATABASE, MAX_FILE_SIZE_KB, VACUUM_INTERVAL_SEC)?;
    let source_db = connect(SOURCE_DATABASE, MAX_FILE_SIZE_KB, VACUUM_INTERVAL_SEC)?;
    let master_cfg = pms_master::config::Config {
        host: CONFIG.host.host.clone(),
        host_pass: CONFIG.host.host_pass.clone(),
    };
    let (event_tx, event_rx) = unbounded();
    let handler_tx = serve(master_cfg, event_tx).await;
    let state = WebState {
        handler_tx: handler_tx.clone(),
    };
    let judge_man = Arc::new(Mutex::new(JudgeMan {
        judge_tx: HashMap::new(),
    }));
    let data = Arc::new(WebData {
        state,
        judge_man: judge_man.clone(),
        judge_db: Arc::new(Mutex::new(judge_db)),
        source_db: Arc::new(Mutex::new(source_db)),
    });
    spawn(async move {
        info!("starting master handler");
        master_handler(judge_man, handler_tx, event_rx).await;
    });
    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new(CONFIG.redis.url.clone())
        .await
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .wrap(middleware::Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone()
            ))
            .service(
                web::scope("/api")
                .service(
                    web::scope("/accounts")
                    .service(api::accounts::login)
                    .service(api::accounts::get_self)
                    .service(api::accounts::delete_self)
                )
                .service(
                    web::scope("/handshake")
                    .service(api::handshake::ping)
                )
            )
    })
    .bind(CONFIG.web.host.clone())?
    .run()
    .await
}
