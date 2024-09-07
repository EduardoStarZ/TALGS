/*
 *
 * app.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */


use ntex::web;
use ntex_session::Session;
use askama::Template;
use crate::database::{app::Purchase, connection::AuthPool, users::{get, User}};
use super::reqwestify;
use crate::session::controller::check_token;
use crate::colors::color::Color;
use crate::database::app::Article;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate{
    auth_level: i16,
    articles: Vec<Article>,
    purchases: Vec<Purchase>
}

#[web::get("/")]
pub async fn home(session: Session, request : web::HttpRequest, auth_pool: web::types::State<AuthPool>) -> web::HttpResponse {
    reqwestify(request);

    let session_info : (bool, Option<String>) = check_token(session);

    if session_info.0 {
        let mut connection = match auth_pool.pool.get() {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err.to_string().warning());
                return web::HttpResponse::InternalServerError().body("could not retrieve a connection to the database.")
            }
        };

        let user : Option<User> = get(&session_info.1.unwrap(), &mut connection);
        
        match user {
            Some(value) => {
                return web::HttpResponse::Ok().body(HomeTemplate{auth_level: value.group, articles: Vec::new(), purchases: Vec::new()}.render().unwrap());
            },
            None => {
                return web::HttpResponse::Unauthorized().body("You are not authorized to see this page")
            }
        }
    }

    return web::HttpResponse::Unauthorized().body("You are not authorized to see this page");
}

 
