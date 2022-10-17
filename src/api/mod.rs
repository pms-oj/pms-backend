pub mod accounts;
pub mod constants;
pub mod graphql;
pub mod handshake;

use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseBlock<T> {
    pub status: bool,
    pub body: T,
}

#[Object]
impl<T> ResponseBlock<T>
where
    T: OutputType + Clone,
{
    async fn status(&self) -> bool {
        self.status
    }

    async fn body(&self) -> T {
        self.body.clone()
    }
}
