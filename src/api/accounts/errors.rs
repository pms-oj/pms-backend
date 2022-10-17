use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(Error, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Enum)]
pub enum AccountError {
    #[error("Ok")]
    None,
    #[error("You aren't logged in")]
    NotLoggedIn,
    #[error("You're already logged in")]
    AlreadyLoggedIn,
    #[error("Some database error occurred")]
    DatabaseError,
    #[error("You're password is not matched")]
    PassNotMatched,
    #[error("Requested user id already exists")]
    UserNotExists,
}

impl ErrorExtensions for AccountError {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            _ => {}
        })
    }
}
