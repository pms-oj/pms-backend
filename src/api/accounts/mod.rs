pub mod errors;

use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use super::constants::*;
use super::ResponseBlock;
use crate::db::accounts;
use crate::middlewares::postgresql::establish_connection;
use errors::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub id: String,
    pub password: String,
}

#[get("/self")]
pub async fn get_self(request: HttpRequest, user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        let id = user.id().unwrap();
        user.logout();
        Identity::login(&request.extensions(), id).ok();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(ResponseBlock {
                status: true,
                body: AccountError::None,
            })
    } else {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(ResponseBlock {
                status: false,
                body: AccountError::NotLoggedIn,
            })
    }
}

#[delete("/self")]
pub async fn delete_self(user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        user.logout();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(ResponseBlock {
                status: true,
                body: AccountError::None,
            })
    } else {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(ResponseBlock {
                status: false,
                body: AccountError::NotLoggedIn,
            })
    }
}

#[post("/login")]
pub async fn login(
    user: Option<Identity>,
    request: HttpRequest,
    form: web::Json<LoginRequest>,
) -> HttpResponse {
    if let None = user {
        if let Ok((error, pk)) = accounts::login(form.id.clone(), form.password.clone()) {
            Identity::login(&request.extensions(), pk.to_string()).ok();
            HttpResponse::Ok()
                .content_type("application/json")
                .json(ResponseBlock {
                    status: true,
                    body: error,
                })
        } else {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(ResponseBlock {
                    status: false,
                    body: AccountError::DatabaseError,
                })
        }
    } else {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(ResponseBlock {
                status: false,
                body: AccountError::AlreadyLoggedIn,
            })
    }
}
