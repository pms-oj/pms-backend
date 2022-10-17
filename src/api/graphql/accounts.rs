use crate::api::accounts::*;
use actix_identity::Identity;
use actix_web::{get, route, web};
use async_graphql::*;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use chrono_tz::Tz;
use uuid::Uuid;

#[derive(Clone, Debug, InputObject)]
pub struct RegisterRequest {
    pub id: String,
    pub pass: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timezone: Option<Tz>,
    pub preferred_language: Option<Uuid>,
}

pub struct UserGql {
    pub pk: Uuid,
    pub id: String,
    pub permission: AccountPerm,
    pub timezone: Tz,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_language: Uuid,
}

#[Object]
impl UserGql {
    async fn pk(&self) -> Uuid {
        self.pk
    }

    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn permission(&self) -> AccountPerm {
        self.permission
    }

    async fn timezone(&self) -> Tz {
        self.timezone
    }

    async fn first_name(&self) -> String {
        self.first_name.clone()
    }

    async fn last_name(&self) -> String {
        self.last_name.clone()
    }

    async fn email(&self) -> String {
        self.email.clone()
    }

    async fn preferred_language(&self) -> Uuid {
        self.preferred_language
    }
}
