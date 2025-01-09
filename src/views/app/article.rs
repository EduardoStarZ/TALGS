/*
 *
 * article.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use ntex::web;
use crate::database::app::article::{self, Article};
use crate::database::connection::AppPool;
use super::super::reqwestify;
use diesel::prelude::*;

#[web::get("/article")]
pub async fn article_route(request : web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().finish();
}

#[web::get("/article")]
pub async fn articles_reader(request : web::HttpRequest, pool : web::types::State<AppPool>) -> web::HttpResponse {
   reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    let articles : Vec<Article> = article::get_all(connection);

    let response_string = articles.iter().map(|x| format!("{:?}", x)).collect::<String>();

    return web::HttpResponse::Ok().body(response_string);
}

#[web::get("/article-{article}")]
pub async fn supplier_reader(request : web::HttpRequest, path : web::types::Path<i32>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let target : i32 = *path;

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let article : Article = article::get(&target, connection).unwrap();

    return web::HttpResponse::Ok().body(format!("{:?}", article));
}

#[web::put("/article")]
pub async fn create_article(request : web::HttpRequest, form : web::types::Form<Article>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    article::create(&*form, connection);

    return web::HttpResponse::Ok().finish(); 
}

#[web::patch("/article")]
pub async fn update_article(request : web::HttpRequest, form : web::types::Form<Article> , pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    article::update(&*form, connection);

    return web::HttpResponse::Ok().finish();
}

#[web::delete("/article")]
pub async fn delete_article(request : web::HttpRequest, form : web::types::Form<i32> , pool : web::types::State<AppPool>) -> web::HttpResponse {

    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    article::delete(&*form, connection);
    
    return web::HttpResponse::Ok().finish();
}
