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

use talgs::hasher::{self, env};
use talgs::session::controller::get_info_handler;
use talgs::database::connection::*;
use ntex::web::{self, middleware::Logger};
use ntex_session::CookieSession;
use ntex_files as fs;
use talgs::files::fs as filesystem;
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

///This is the main function of the cargo project
#[ntex::main]
pub async fn main() -> std::io::Result<()> {

    filesystem::check_dir_existance(); 

    let _ : () = match env::get_hash() {
        Some(_) => (),
        None => env::set_hash(hasher::hash::create_hash())
    };

    let address : String = match env::get_value("IP") {
        Some(value) => value,
        None => {
            env::set_value("IP", "127.0.0.1");
            "127.0.0.1".to_string()
        }
    };

    let port : u16 = match env::get_value("PORT") {
        Some(value) => value.trim().parse::<u16>().unwrap(),
        None => {
            env::set_value("PORT", "8080");
            8080
        }
    };

    //let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    //builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();

    //builder.set_certificate_chain_file("cert.pem").unwrap();

    let app_pool = AppPool {pool: create_pool(create_connection("app.sqlite3"))};
    let auth_pool = AuthPool {user_pool: create_pool(create_connection("auth.sqlite3")), key_pool: create_pool(create_connection("key.sqlite3"))};

    web::HttpServer::new(move || {
        web::App::new()
            .state(app_pool.clone())
            .state(auth_pool.clone())
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(talgs::views::get_test_route)
            .service(talgs::views::post_test_route)
            .service(talgs::forms::product::create)
            .service(talgs::forms::product::update)
            .service(talgs::forms::product::delete)
            .service(
                web::scope("/api")
                .service(talgs::views::app::product::product_reader)
                .service(talgs::views::app::product::create_product)
                .service(talgs::views::app::product::update_product)
                .service(talgs::views::app::product::delete_product)

                .service(talgs::views::app::supplier::supplier_reader)
                .service(talgs::views::app::supplier::suppliers_reader)
                .service(talgs::views::app::supplier::create_supplier)
                .service(talgs::views::app::supplier::delete_supplier)
                .service(talgs::views::app::supplier::update_supplier)

                .service(talgs::views::app::stock::create_stock)
                .service(talgs::views::app::stock::delete_stock)
                .service(talgs::views::app::stock::update_stock)
                .service(talgs::views::app::stock::stock_reader)
                .service(talgs::views::app::stock::stocks_reader)
            

                .service(talgs::views::app::purchase::create_purchase)
                .service(talgs::views::app::purchase::delete_purchase)
                .service(talgs::views::app::purchase::update_purchase)
                .service(talgs::views::app::purchase::purchase_reader)
                .service(talgs::views::app::purchase::purchases_reader)
                

                .service(talgs::views::app::address::create_address)
                .service(talgs::views::app::address::update_address)
                .service(talgs::views::app::address::address_reader)
                .service(talgs::views::app::address::addresses_reader)
                .service(talgs::views::app::address::delete_address)


                .service(talgs::views::app::article::create_article)
                .service(talgs::views::app::article::update_article)
                .service(talgs::views::app::article::delete_article)
                .service(talgs::views::app::article::articles_reader)
                .service(talgs::views::app::article::article_reader)


                .service(talgs::views::app::category::create_category)
                .service(talgs::views::app::category::update_category)
                .service(talgs::views::app::category::delete_category)
                .service(talgs::views::app::category::category_reader)
                .service(talgs::views::app::category::categories_reader)
                )
            .service(get_info_handler)
            .service(fs::Files::new("/static", "static/").show_files_listing())
            .service( 
                web::scope("/auth")
                .service(talgs::views::auth::login)
                .service(talgs::views::auth::login_form)
                .service(talgs::views::auth::register)
                .service(talgs::views::auth::register_form)
                )
    })
    //.bind_openssl((address, port), builder)?
    .bind((address, port))?
    .run()
    .await
}
