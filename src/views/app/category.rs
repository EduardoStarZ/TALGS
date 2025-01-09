/*
 *
 * supplier.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use ntex::web;
use crate::database::app::category::{self, Category};
use crate::database::connection::AppPool;
use super::super::reqwestify;
use diesel::prelude::*;

#[web::get("/category")]
pub async fn category_route(request : web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().finish();
}

#[web::get("/categories")]
pub async fn categories_reader(request : web::HttpRequest, pool : web::types::State<AppPool>) -> web::HttpResponse {
   reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    let categories : Vec<Category> = category::get_all(connection);

    let response_string = categories.iter().map(|x| format!("{:?}", x)).collect::<String>();

    return web::HttpResponse::Ok().body(response_string);
}

#[web::get("/category-{category}")]
pub async fn category_reader(request : web::HttpRequest, path : web::types::Path<i16>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let target : i16 = *path;

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let category : Category = category::get(&target, connection).unwrap();

    return web::HttpResponse::Ok().body(format!("{:?}", category));
}

#[web::put("/category")]
pub async fn create_category<'a>(request : web::HttpRequest, form : web::types::Form<Category<'a>>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    category::create(&*form, connection);

    return web::HttpResponse::Ok().finish(); 
}

#[web::patch("/category")]
pub async fn update_category<'a>(request : web::HttpRequest, form : web::types::Form<Category<'a>> , pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    category::update(&*form, connection);

    return web::HttpResponse::Ok().finish();
}

#[web::delete("/category")]
pub async fn delete_category(request : web::HttpRequest, form : web::types::Form<i16> , pool : web::types::State<AppPool>) -> web::HttpResponse {

    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    category::delete(&*form, connection);
    
    return web::HttpResponse::Ok().finish();
}
