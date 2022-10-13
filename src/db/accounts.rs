use diesel::prelude::*;
use sha3::{Digest, Sha3_256};
use uuid::Uuid;

use super::models::*;
use super::schema::*;
use crate::api::accounts::errors::AccountError;
use crate::middlewares::postgresql::establish_connection;

pub fn login(id: String, pass: String) -> QueryResult<(AccountError, Uuid)> {
    let mut db = establish_connection();
    let mut hasher = Sha3_256::new();
    hasher.update(pass.as_bytes());
    let pass_hashed = hasher.finalize();
    let mut items = users::table
        .filter(users::dsl::id.eq(id))
        .load::<User>(&mut db)?;
    if let Some(user) = items.pop() {
        if hex::encode(pass_hashed) == user.pass {
            Ok((AccountError::None, user.pk))
        } else {
            Ok((AccountError::PassNotMatched, Uuid::nil()))
        }
    } else {
        Ok((AccountError::UserNotExists, Uuid::nil()))
    }
}
