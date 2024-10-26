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
use crate::database::{app::{Purchase, Article, Product}, users::{get, User}};
use super::reqwestify;
use crate::session::controller::check_token;
use crate::colors::color::Color;
use diesel::prelude::*;
use crate::schema::app::*;
use crate::files::receiver::read_payload_to_string;
use crate::str::filter::{Form, payload_into_values};
use crate::files::fs::{self, rand_name};
use crate::auth::parser::unspaced_hex_str_to_u8_vec;
use std::borrow::Cow;


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

        let email : String = session_info.1.unwrap();

        let user : Option<User> = get(email.as_str(), &mut connection);
        
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
pub async fn create_product_receiver(request : web::HttpRequest, payload : web::types::Payload, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let payload : String = match read_payload_to_string(payload).await {
        Some(value) => value,
        None => return web::HttpResponse::BadRequest().finish()
    };

    println!("{payload}");

    let values : Vec<Form> = payload_into_values(&payload);

    let mut product : Product = Product {
        id: 134536,
        name: Cow::from(""),
        price: 0.0,
        id_category: 0,
        image: Cow::from(""),
        total_amount: 0
    };

    let mut filename : String = String::new();

    for x in values {
        match x.name {
            "file" => {
                filename = format!("{}.{}", rand_name(), x.value
                        .chars()
                        .rev()
                        .collect::<String>()
                        .split_once(".")
                        .unwrap()
                        .0
                        .chars()
                        .rev()
                        .collect::<String>()
                        );

                fs::create_file(&filename.clone());
            },
            "bytes" => {
                fs::write_contents(&unspaced_hex_str_to_u8_vec(&String::from(x.value)), &filename);
            },
            "name" => {
                product.name = Cow::from(x.value);
            },
            "price" => {
                product.price = x.value.trim().parse::<f32>().unwrap();
            },
            "category" => {
                product.id_category = x.value.trim().parse::<i16>().unwrap();
            },
            _ => return web::HttpResponse::Forbidden().finish()
        } 
    }

    diesel::insert_into(product::table)
        .values(&product)
        .execute(&mut pool.pool.get().unwrap())
        .unwrap();

    return web::HttpResponse::Ok().finish(); 
}
