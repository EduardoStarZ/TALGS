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
use crate::database::app::supplier::{self, Supplier};
use crate::database::connection::AppPool;
use super::super::reqwestify;
use diesel::prelude::*;

#[web::get("/supplier")]
pub async fn suppliers_reader(request : web::HttpRequest, pool : web::types::State<AppPool>) -> web::HttpResponse {
   reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    let suppliers : Vec<Supplier> = supplier::get_all(connection);

    let response_string = suppliers.iter().map(|x| format!("{:?}", x)).collect::<String>();

    return web::HttpResponse::Ok().body(response_string);
}

#[web::get("/supplier-{supplier}")]
pub async fn supplier_reader(request : web::HttpRequest, path : web::types::Path<i32>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let target : i32 = *path;

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let supplier : Supplier = supplier::get(&target, connection).unwrap();

    return web::HttpResponse::Ok().body(format!("{:?}", supplier));
}

#[web::put("/supplier")]
pub async fn create_supplier<'a>(request : web::HttpRequest, form : web::types::Form<Supplier<'a>>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    supplier::create(&*form, connection);

    return web::HttpResponse::Ok().finish(); 
}

#[web::patch("/supplier")]
pub async fn update_supplier<'a>(request : web::HttpRequest, form : web::types::Form<Supplier<'a>> , pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    supplier::update(&*form, connection);

    return web::HttpResponse::Ok().finish();
}

#[web::delete("/supplier")]
pub async fn delete_supplier(request : web::HttpRequest, form : web::types::Form<i32> , pool : web::types::State<AppPool>) -> web::HttpResponse {

    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    supplier::delete(&*form, connection);
    
    return web::HttpResponse::Ok().finish();
}
