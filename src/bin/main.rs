use talgs::{index, form};
use talgs::session::controller::{login_handler, get_info_handler};
use ntex::web::{self, middleware::Logger};
use ntex_session::CookieSession;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use r2d2::Pool;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let adress : &str = "127.0.0.1";
    let port : u16 = 8080;

    let default_msg : &str = "Database URL must be a valid path to a SQLite file";

    let key_manager = ConnectionManager::<SqliteConnection>::new("key.sqlite3");
    let app_manager = ConnectionManager::<SqliteConnection>::new("app.sqlite3");
    let auth_manager = ConnectionManager::<SqliteConnection>::new("auth.sqlite3");

    let key_pool = Pool::builder()
        .build(key_manager)
        .expect(default_msg);

    let app_pool = Pool::builder()
        .build(app_manager)
        .expect(default_msg);

    let auth_pool = Pool::builder()
        .build(auth_manager)
        .expect(default_msg);

    web::HttpServer::new(move || {
        web::App::new()
            .state(key_pool.clone())
            .state(app_pool.clone())
            .state(auth_pool.clone())
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(index)
            .service(login_handler)
            .service(get_info_handler)
            .service(form)
    })
    .bind((adress, port))?
    .run()
    .await
}
