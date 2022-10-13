use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::CONFIG;

pub fn establish_connection() -> PgConnection {
    let config = CONFIG.clone();
    PgConnection::establish(&config.database.url)
        .expect(&format!("Error connecting to {}", config.database.url))
}
