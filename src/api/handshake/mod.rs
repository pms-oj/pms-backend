use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use super::constants::*;
use super::ResponseBlock;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ResponseBlock {
            status: true,
            body: String::from(PONG),
        })
}
