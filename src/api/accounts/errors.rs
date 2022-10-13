use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AccountError {
    None,
    NotLoggedIn,
    AlreadyLoggedIn,
    DatabaseError,
    PassNotMatched,
    UserNotExists,
}
