use diesel::{Connection, SqliteConnection};
use std::env;

pub fn create_connection_to_auth() -> SqliteConnection {
    let database_url = match env::var("AUTH_URL") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error fetching environment variable: {err}");
            "auth.sqlite3".to_string()
        }
    };

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}, shutting down...", database_url))
}
