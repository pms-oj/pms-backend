pub mod accounts;

use crate::api::accounts::*;
use crate::api::ResponseBlock;
use crate::db::accounts::*;
use crate::db::models::NewUser;
use crate::CONFIG;

use accounts::*;

use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::*;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sha3::{Digest, Sha3_256};
use uuid::Uuid;

pub struct QueryRoot;

pub async fn gql_playground() -> HttpResponse {
    if CONFIG.web.enable_gql_playground {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/api/gql").subscription_endpoint("/api/gql"),
            ))
    } else {
        HttpResponse::Forbidden().body(())
    }
}

pub async fn gql_endpoint(
    schema: web::Data<Schema<QueryRoot, Mutation, EmptySubscription>>,
    user: Option<Identity>,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut gql_req = gql_req.into_inner();
    if let Some(user) = user {
        gql_req = gql_req.data(user.id().unwrap());
    }
    schema.execute(gql_req).await.into()
}

#[Object]
impl QueryRoot {
    async fn info<'ctx>(&self, ctx: &'ctx Context<'_>) -> Result<UserGql, AccountError> {
        if let Some(pk) = ctx.data_opt::<String>() {
            let pk = pk.parse::<Uuid>().unwrap();
            let from_db = find_user(pk).unwrap();
            Ok(UserGql {
                pk,
                id: from_db.id.clone(),
                permission: from_db.permission.try_into().unwrap(),
                timezone: from_db.timezone().unwrap(),
                first_name: from_db.first_name,
                last_name: from_db.last_name,
                email: from_db.email,
                preferred_language: from_db.preferred_language,
            })
        } else {
            Err(AccountError::NotLoggedIn)
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register(&self, register_req: RegisterRequest) -> ResponseBlock<AccountError> {
        if let Ok(_) = by_id(register_req.id.clone()) {
            ResponseBlock {
                status: false,
                body: AccountError::UserNotExists,
            }
        } else {
            let mut hasher = Sha3_256::new();
            hasher.update(register_req.pass.as_bytes());
            let pass_hashed = hex::encode(hasher.finalize());
            let user = NewUser {
                id: register_req.id,
                pass: pass_hashed,
                permission: AccountPerm::User as i32,
                timezone: register_req
                    .timezone
                    .unwrap_or_else(|| CONFIG.general.timezone)
                    .to_string(),
                first_name: register_req.first_name,
                last_name: register_req.last_name,
                email: register_req.email,
                preferred_language: register_req
                    .preferred_language
                    .unwrap_or_else(|| CONFIG.general.default_language),
            };
            match register(user) {
                Ok(_) => ResponseBlock {
                    status: true,
                    body: AccountError::None,
                },
                Err(err) => ResponseBlock {
                    status: false,
                    body: AccountError::DatabaseError,
                },
            }
        }
    }
}
