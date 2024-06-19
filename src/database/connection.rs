use diesel::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use r2d2::Pool;


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
