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
use crate::database::connection::{AppPool, AuthPool};
use crate::database::{app::{Purchase, Article}, users::{get, User}};
use super::reqwestify;
use crate::session::controller::check_token;
use crate::colors::color::Color;
use diesel::prelude::*;
use crate::schema::app::*;
use crate::files::receiver::read_payload_to_string;
use crate::str::filter::{Form, payload_into_values};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate{
    auth_level: i16,
    articles: Vec<Article>,
    purchases: Vec<Purchase>
}

#[web::get("/")]
pub async fn home(session: Session, request : web::HttpRequest, app_pool: web::types::State<AppPool>, auth_pool: web::types::State<AuthPool>) -> web::HttpResponse {
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
            Some(user) => {
                let cors_purchases : Vec<Purchase> = purchase::table
                    .filter(purchase::id_user.eq(user.id as i32))
                    .select(Purchase::as_select())
                    .load(&mut app_pool.pool.get().unwrap())
                    .unwrap();

                let mut cors_articles : Vec<Article> = Vec::new();

                for purchase in &cors_purchases {
                    let mut articles : Vec<Article> = article::table
                        .filter(article::id_purchase.eq(purchase.id))
                        .select(Article::as_select())
                        .load(&mut app_pool.pool.get().unwrap())
                        .unwrap();

                    cors_articles.append(&mut articles);
                }

                return web::HttpResponse::Ok().body(HomeTemplate{auth_level: user.group, articles: cors_articles, purchases: cors_purchases}.render().unwrap());
            },
            None => {
                return web::HttpResponse::Unauthorized().body("You are not authorized to see this page")
            }
        }
    }

    return web::HttpResponse::Unauthorized().body("You are not authorized to see this page");
}

#[derive(Template)]
#[template(path="new_product.html")]
struct NewProductpage {}

#[web::get("/product/create")]
pub async fn create_product_route(request : web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().body(NewProductpage{}.render().unwrap());
}

#[web::post("/product/create")]
pub async fn create_product_receiver(request : web::HttpRequest, payload : web::types::Payload) -> web::HttpResponse {
    reqwestify(request);

    let payload : String = match read_payload_to_string(payload).await {
        Some(value) => value,
        None => return web::HttpResponse::BadRequest().finish()
    };

    println!("{payload}");

    let values : Vec<Form> = payload_into_values(&payload);

    for x in values {
        println!("{} - {}",x.name, x.value);
    }

    return web::HttpResponse::Ok().finish(); 
}
