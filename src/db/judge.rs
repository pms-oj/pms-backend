use diesel::prelude::*;
use uuid::Uuid;

use super::models::*;
use super::schema::*;
use crate::middlewares::postgresql::establish_connection;

pub fn find_submission(uuid: Uuid) -> QueryResult<Submission> {
    let mut db = establish_connection();
    submissions::table.find(uuid).first(&mut db)
}
