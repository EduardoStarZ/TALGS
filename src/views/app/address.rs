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
use crate::database::app::address::{self, Address};
use crate::database::connection::AppPool;
use super::super::reqwestify;
use diesel::prelude::*;

#[web::get("/address")]
pub async fn addresses_reader(request : web::HttpRequest, pool : web::types::State<AppPool>) -> web::HttpResponse {
   reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    let addresses : Vec<Address> = address::get_all(connection);

    let response_string = addresses.iter().map(|x| format!("{:?}", x)).collect::<String>();

    return web::HttpResponse::Ok().body(response_string);
}

#[web::get("/address-{address}")]
pub async fn address_reader(request : web::HttpRequest, path : web::types::Path<i32>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let target : i32 = *path;

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let address : Address = address::get(&target, connection).unwrap();

    return web::HttpResponse::Ok().body(format!("{:?}", address));
}

#[web::put("/address")]
pub async fn create_address<'a>(request : web::HttpRequest, form : web::types::Form<Address<'a>>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    address::create(&*form, connection);

    return web::HttpResponse::Ok().finish(); 
}

#[web::patch("/address")]
pub async fn update_address<'a>(request : web::HttpRequest, form : web::types::Form<Address<'a>> , pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    address::update(&*form, connection);

    return web::HttpResponse::Ok().finish();
}

#[web::delete("/address")]
pub async fn delete_address(request : web::HttpRequest, form : web::types::Form<i32> , pool : web::types::State<AppPool>) -> web::HttpResponse {

    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    address::delete(&*form, connection);
    
    return web::HttpResponse::Ok().finish();
}
