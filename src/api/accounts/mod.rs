pub mod errors;

use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse};
use async_graphql::*;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::convert::TryFrom;

use super::constants::*;
use super::ResponseBlock;
use crate::db::accounts;
use crate::middlewares::postgresql::establish_connection;

pub use errors::*;

#[derive(Enum, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
#[repr(i32)]
pub enum AccountPerm {
    User = 0x00,
    Admin = 0xFF,
}

impl TryFrom<i32> for AccountPerm {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::User as i32 => Ok(Self::User),
            x if x == Self::Admin as i32 => Ok(Self::Admin),
            _ => Err(()),
        }
    }
}

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
        let mut hasher = Sha3_256::new();
        hasher.update(form.password.as_bytes());
        let pass_hashed = hex::encode(hasher.finalize());
        if let Ok((error, pk)) = accounts::login(form.id.clone(), pass_hashed) {
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
