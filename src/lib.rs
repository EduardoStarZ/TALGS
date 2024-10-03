/*
 *
 * lib.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

pub mod auth;
pub mod session;
pub mod database;
pub mod schema;
pub mod views;
pub mod colors;
pub mod hasher;
pub mod files;

use ntex::web;
use askama::Template;

#[derive(Template)]
#[template(path="form.html")]
struct FormTemplate{}

#[web::get("/")]
pub async fn index() -> web::HttpResponse {
    return web::HttpResponse::Ok().body("Hello world");
}

#[web::get("/form")]
pub async fn form() -> web::HttpResponse {
    return web::HttpResponse::Ok().body(FormTemplate{}.render().unwrap())
}
