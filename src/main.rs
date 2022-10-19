#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate thiserror;

mod api;
mod config;
mod constants;
mod contests;
mod db;
mod judge;
mod middlewares;
mod tasks;

#[cfg(test)]
mod tests;

use actix::dev::ToEnvelope;
use actix::prelude::*;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, guard, middleware, web, App, HttpServer};
use async_graphql::*;
use async_std::channel::{unbounded, Sender};
use async_std::sync::{Arc, Mutex};
use async_std::task::spawn;
use ckydb::{connect, Controller};
use judge_protocol::judge::*;
use log::*;
use pms_master::event::*;
use pms_master::handler::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use uuid::Uuid;

use crate::config::*;
use crate::constants::*;
use crate::db::keydb::*;
use crate::judge::*;

lazy_static! {
    static ref CONFIG: Config = {
        let s = read_to_string(CONFIG_FILE).expect("Some error occured");
        info!("Loaded PMS backend config file");
        toml::from_str(&s).expect("Some error occured")
    };
    static ref APPDATA: Arc<WebData> = {
        log4rs::init_file(LOG_CONFIG_FILE, Default::default()).unwrap();
        info!("pms-backend {}", env!("CARGO_PKG_VERSION"));
        let db_threads = CONFIG.general.db_threads.unwrap_or_else(|| num_cpus::get());
        info!(
            "Starting key-based database service of {} threads",
            db_threads
        );
        let source_db = KeyDbService::start(SOURCE_DATABASE, db_threads);
        let judge_db = KeyDbService::start(JUDGE_DATABASE, db_threads);
        let master_cfg = pms_master::config::Config {
            host: CONFIG.host.host.clone(),
            host_pass: CONFIG.host.host_pass.clone(),
        };
        let judge_service = JudgeService {
            judge_addrs: HashMap::new(),
        };
        let judge_addr = judge_service.start();
        let handler_service = HandlerService {
            cfg: master_cfg,
            event_addr: judge_addr.clone(),
            state: None,
        };
        let handler_addr = handler_service.start();
        let state = WebState { handler_addr };
        Arc::new(WebData {
            state,
            judge_addr: judge_addr,
            source_db,
            judge_db,
        })
    };
}

#[derive(Clone)]
pub struct WebState {
    pub handler_addr: Addr<HandlerService<JudgeService>>,
}

#[derive(Clone)]
pub struct WebData {
    pub state: WebState,
    pub judge_addr: Addr<JudgeService>,
    pub source_db: Addr<KeyDbService>,
    pub judge_db: Addr<KeyDbService>,
}

impl WebData {
    fn touch(&self) {
        println!("WebData is touched!");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    APPDATA.touch();
    if CONFIG.web.enable_gql_playground {
        info!(
            "GraphiQL IDE is arrived: http://{}/api/gql",
            CONFIG.web.host.clone()
        );
    }
    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new(CONFIG.redis.url.clone())
        .await
        .expect(&format!(
            "Unable to connect redis {}",
            CONFIG.redis.url.clone()
        ));
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .service(
                web::scope("/api")
                    .app_data(web::Data::new(Schema::new(
                        api::graphql::QueryRoot,
                        api::graphql::Mutation,
                        EmptySubscription,
                    )))
                    .service(
                        web::scope("/accounts")
                            .service(api::accounts::login)
                            .service(api::accounts::get_self)
                            .service(api::accounts::delete_self),
                    )
                    .service(web::scope("/handshake").service(api::handshake::ping))
                    .service(
                        web::resource("/gql")
                            .guard(guard::Post())
                            .to(api::graphql::gql_endpoint),
                    )
                    .service(
                        web::resource("/gql")
                            .guard(guard::Get())
                            .to(api::graphql::gql_playground),
                    ),
            )
    })
    .bind(CONFIG.web.host.clone())?
    .run()
    .await
}
