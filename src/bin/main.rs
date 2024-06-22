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
use talgs::session::controller::{login_handler, get_info_handler};
use talgs::database::connection::*;
use ntex::web::{self, middleware::Logger};
use ntex_session::CookieSession;
use ntex_files as fs;

///This is the main function of the cargo project
#[ntex::main]
async fn main() -> std::io::Result<()> {
    let adress : &str = "127.0.0.1";
    let port : u16 = 8080;

    let key_pool = create_pool(create_connection("key.sqlite3"));
    let app_pool = create_pool(create_connection("app.sqlite3"));
    let auth_pool = create_pool(create_connection("auth.sqlite3"));

    web::HttpServer::new(move || {
        web::App::new()
            .state(key_pool.clone())
            .state(app_pool.clone())
            .state(auth_pool.clone())
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(true))
            .service(index)
            .service(login_handler)
            .service(get_info_handler)
            .service(form)
            .service(fs::Files::new("/static", "static/").show_files_listing())
            .service( 
                web::scope("/auth")
                .service(talgs::views::auth::login)
                ) 
    })
    .bind((adress, port))?
    .run()
    .await
}
