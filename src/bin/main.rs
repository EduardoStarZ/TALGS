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

use talgs::colors::color::Color;
use talgs::{index, form};
use talgs::session::controller::get_info_handler;
use talgs::database::connection::*;
use ntex::web::{self, middleware::Logger};
use ntex_session::CookieSession;
use ntex_files as fs;
use std::env::args;
use talgs::database::users;

///This is the main function of the cargo project
#[ntex::main]
pub async fn main() -> std::io::Result<()> {

    if args().len() > 1 {
        match args().collect::<Vec<String>>()[1].as_str() {
            "db_print" => {
                for x in users::get_all(&mut create_pure_connection("auth.sqlite3")) {
                    let data : String = format!("||\n|| ID: {}\n|| Name: {}\n|| Email: {}\n||", x.id, x.name, x.email);        
                    println!("{}", data.database_values());
                }

                panic!("");
            },
            _ => ()
        }
    }

    

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
                .service(talgs::views::auth::register_form)
                )
    })
    .bind((adress, port))?
    .run()
    .await
}
