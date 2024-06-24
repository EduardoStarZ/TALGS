/*
 *
 * main.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use talgs::{index, form};
use talgs::session::controller::get_info_handler;
use talgs::database::connection::*;
use ntex::web::{self, middleware::Logger};
use ntex_session::CookieSession;
use ntex_files as fs;

///This is the main function of the cargo project
#[ntex::main]
pub async fn main() -> std::io::Result<()> {
    let adress : &str = "127.0.0.1";
    let port : u16 = 8080;

    let key_pool = KeyPool {pool: create_pool(create_connection("key.sqlite3"))};
    let app_pool = AppPool {pool: create_pool(create_connection("app.sqlite3"))};
    let auth_pool = AuthPool {pool: create_pool(create_connection("auth.sqlite3"))};

    web::HttpServer::new(move || {
        web::App::new()
            .state(key_pool.clone())
            .state(app_pool.clone())
            .state(auth_pool.clone())
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(true))
            .service(index)
            .service(get_info_handler)
            .service(form)
            .service(fs::Files::new("/static", "static/").show_files_listing())
            .service( 
                web::scope("/auth")
                .service(talgs::views::auth::login)
                .service(talgs::views::auth::login_form)
                .service(talgs::views::auth::register)
            )
    })
    .bind((adress, port))?
    .run()
    .await
}
