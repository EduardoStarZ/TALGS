pub mod schema;
pub mod models;
pub mod section;
pub mod product;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn establish_local_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_remote_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("REMOTE_DATABASE_URL").expect("REMOTE_DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn counter() -> Vec<u128> {
    vec![
        SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos(),

        SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros(),
        
        SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
    ]
}
