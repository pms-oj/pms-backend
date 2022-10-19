use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}
