/*
 *
 * purchase.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use ntex::web;
use crate::database::app::purchase::{self, Purchase};
use crate::database::connection::AppPool;
use super::super::reqwestify;
use diesel::prelude::*;

#[web::get("/purchase")]
pub async fn purchase_route(request : web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().finish();
}

#[web::get("/purchase")]
pub async fn purchases_reader(request : web::HttpRequest, pool : web::types::State<AppPool>) -> web::HttpResponse {
   reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    let purchases : Vec<Purchase> = purchase::get_all(connection);

    let response_string = purchases.iter().map(|x| format!("{:?}", x)).collect::<String>();

    return web::HttpResponse::Ok().body(response_string);
}

#[web::get("/purchase-{purchase}")]
pub async fn purchase_reader(request : web::HttpRequest, path : web::types::Path<i32>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let target : i32 = *path;

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let purchase : Purchase = purchase::get(&target, connection).unwrap();

    return web::HttpResponse::Ok().body(format!("{:?}", purchase));
}

#[web::put("/purchase")]
pub async fn create_purchase<'a>(request : web::HttpRequest, form : web::types::Form<Purchase<'a>>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    purchase::create(&*form, connection);

    return web::HttpResponse::Ok().finish(); 
}

#[web::patch("/purchase")]
pub async fn update_purchase<'a>(request : web::HttpRequest, form : web::types::Form<Purchase<'a>> , pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    purchase::update(&*form, connection);

    return web::HttpResponse::Ok().finish();
}

#[web::delete("/address")]
pub async fn delete_purchase(request : web::HttpRequest, form : web::types::Form<i32> , pool : web::types::State<AppPool>) -> web::HttpResponse {

    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    purchase::delete(&*form, connection);
    
    return web::HttpResponse::Ok().finish();
}
