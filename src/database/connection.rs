use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

/// Creates a new connection
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("Unable to establish connection")
}