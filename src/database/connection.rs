/*
 *
 * connection.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use diesel::{Connection, SqliteConnection};
use diesel::r2d2::{self, ConnectionManager};
use r2d2::Pool;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct KeyPool {
    pub pool : DbPool
}

#[derive(Clone)]
pub struct AuthPool {
    pub pool : DbPool
}

#[derive(Clone)]
pub struct AppPool {
    pub pool : DbPool
}

pub fn create_pure_connection(db_url : &str) -> SqliteConnection {
    SqliteConnection::establish(db_url).unwrap_or_else(|_| panic!("Issues setting up database, shutting down"))
}

///Function that takes up a static string reference as a path to a sqlite file and creates a
///connection manager
pub fn create_connection(db_url : &str) -> ConnectionManager<SqliteConnection> {
    ConnectionManager::<SqliteConnection>::new(db_url)
}

///Function that takes up a r2d2 connection manager and creates a database pool
pub fn create_pool(manager : ConnectionManager<SqliteConnection>) -> Pool<ConnectionManager<SqliteConnection>> {
    match Pool::builder()
        .build(manager) {
            Ok(value) => value,
            Err(_) => {
                panic!("Error while setting up connection pool, database URL must be a valid path to a SQLite file")
            }
        }
}
