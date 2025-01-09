/*
 *
 * stock.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use ntex::web;
use crate::database::app::stock::{self, Stock};
use crate::database::connection::AppPool;
use super::super::reqwestify;
use diesel::prelude::*;

#[web::get("/stock")]
pub async fn stock_route(request : web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().finish();
}

#[web::get("/stock")]
pub async fn stocks_reader(request : web::HttpRequest, pool : web::types::State<AppPool>) -> web::HttpResponse {
   reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    let stocks : Vec<Stock> = stock::get_all(connection);

    let response_string = stocks.iter().map(|x| format!("{:?}", x)).collect::<String>();

    return web::HttpResponse::Ok().body(response_string);
}

#[web::get("/stock-{stock}")]
pub async fn stock_reader(request : web::HttpRequest, path : web::types::Path<i32>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let target : i32 = *path;

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let address : Stock = stock::get(&target, connection).unwrap();

    return web::HttpResponse::Ok().body(format!("{:?}", address));
}

#[web::put("/stock")]
pub async fn create_stock<'a>(request : web::HttpRequest, form : web::types::Form<Stock<'a>>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    stock::create(&*form, connection);

    return web::HttpResponse::Ok().finish(); 
}

#[web::patch("/stock")]
pub async fn update_stock<'a>(request : web::HttpRequest, form : web::types::Form<Stock<'a>> , pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    stock::update(&*form, connection);

    return web::HttpResponse::Ok().finish();
}

#[web::delete("/address")]
pub async fn delete_stock(request : web::HttpRequest, form : web::types::Form<i32> , pool : web::types::State<AppPool>) -> web::HttpResponse {

    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    stock::delete(&*form, connection);
    
    return web::HttpResponse::Ok().finish();
}
