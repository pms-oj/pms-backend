pub mod accounts;
pub mod constants;
pub mod handshake;
pub mod restful;
pub mod graphql;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseBlock<T> {
    pub status: bool,
    pub body: T,
}
