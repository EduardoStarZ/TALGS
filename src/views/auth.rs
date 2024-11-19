/*
 *
 * auth.rs
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
use crate::{colors::color::Color, database::{connection::AuthPool, models::ResultCode}, session::controller};
use askama::Template;
use crate::session::model::{LoginInfo, RegisterForm};
use super::reqwestify;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate{}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate{}


#[web::post("/login")]
pub async fn login_form(session : Session, auth_pool : web::types::State<AuthPool>, form : web::types::Form<LoginInfo>, request: web::HttpRequest) -> web::HttpResponse {

    reqwestify(request);

    let mut auth_connection = match auth_pool.user_pool.get() {
        Ok(value) => value,
        Err(err) => {
            println!("Failed to receive a connection to the database: {}", err.to_string().warning());
            return web::HttpResponse::InternalServerError().body("");
        }
    };

    let mut key_connection = match auth_pool.key_pool.get() {
        Ok(value) => value,
        Err(err) => {
            println!("Failed to receive a connection to the database: {}", err.to_string().warning());
            return web::HttpResponse::InternalServerError().body("")
        }
    };

    let response = web::block(move || controller::login_handler(&form, &mut auth_connection, &mut key_connection)).await.unwrap();

    match response.result {
        Some(value) => {
            match value {
                ResultCode::UnauthorizedError => return web::HttpResponse::Unauthorized().body(""),
                _ => return web::HttpResponse::InternalServerError().body("")
            }
        },
        None => {
            match response.token {
                Some(value) => {
                    session.set("Auth-Token", &value).unwrap();
                    return web::HttpResponse::Ok().body(format!("Logged in with token: {value}"));
                },
                None => return web::HttpResponse::InternalServerError().body("")
            } 
        }

    }
}

#[web::get("/login")]
pub async fn login(request: web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);
   
    return web::HttpResponse::Ok().body(LoginTemplate{}.render().unwrap());
}

#[web::get("/register")]
pub async fn register(request: web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);
    
    return web::HttpResponse::Ok().body(RegisterTemplate{}.render().unwrap());
}

#[web::post("/register")]
pub async fn register_form(form : web::types::Form<RegisterForm>, auth_pool : web::types::State<AuthPool>, request: web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);

    if form.password1 != form.password2 {
        return web::HttpResponse::Unauthorized().body(RegisterTemplate{}.render().unwrap())
    }

    let mut auth_connection = match auth_pool.user_pool.get() {
        Ok(value) => value,
        Err(err) => {
            println!("Failed to receive a connection to the database: {}", err.to_string().warning());
            return web::HttpResponse::InternalServerError().body("yuck");
        }
    };

    let mut key_connection = match auth_pool.key_pool.get() {
        Ok(value) => value,
        Err(err) => {
            println!("Failed to receive a connection to the database: {}", err.to_string().warning());
            return web::HttpResponse::InternalServerError().body("yuck")
        }
    };

    let response : bool = web::block(move || controller::register_handler(form, &mut auth_connection, &mut key_connection)).await.unwrap();

    if !response {
        return web::HttpResponse::Unauthorized().body(RegisterTemplate{}.render().unwrap());
    }

    return web::HttpResponse::Ok().body("Created account");
}
