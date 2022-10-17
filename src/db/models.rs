use chrono::prelude::*;
use chrono_tz::Tz;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::*;

#[derive(Clone, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: String,
    pub pass: String,
    pub permission: i32,
    pub timezone: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_language: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub pass: String,
    pub permission: i32,
    pub timezone: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub preferred_language: Uuid,
    pub pk: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "teams"]
pub struct Team {
    pub pk: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "team_users"]
pub struct TeamUser {
    pub pk: Uuid,
    pub user_pk: Uuid,
    pub team_pk: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "tasks"]
pub struct Task {
    pub pk: Uuid,
    pub internal_task_uuid: Uuid,
    pub name: String,
    pub code: String,
    pub is_public: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "contest_tasks"]
pub struct ContestTask {
    pub pk: Uuid,
    pub task_pk: Uuid,
    pub contest_pk: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "contests"]
pub struct Contest {
    pub pk: Uuid,
    pub name: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub is_public: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "contest_accessible_users"]
pub struct ContestAccessibleUser {
    pub pk: Uuid,
    pub user_pk: Uuid,
    pub contest_pk: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "contest_accessible_teams"]
pub struct ContestAccessibleTeam {
    pub pk: Uuid,
    pub team_pk: Uuid,
    pub contest_pk: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "submissions"]
pub struct Submission {
    pub pk: Uuid,
    pub user_pk: Uuid,
    pub task_pk: Uuid,
    pub lang_uuid: Uuid,
    pub issued_at: DateTime<Utc>,
}

impl User {
    pub fn timezone(&self) -> Result<Tz, String> {
        self.timezone.parse::<Tz>()
    }
}
