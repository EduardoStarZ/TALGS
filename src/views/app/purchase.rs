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

use askama::Template;
use ntex::web;
use serde::Deserialize;
use crate::database::app::article::{self, Article};
use crate::database::app::purchase::{self, Purchase};
use crate::database::connection::AppPool;
use crate::Redirect;
use super::super::reqwestify;
use diesel::prelude::*;
use std::borrow::Cow;

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

#[derive(Deserialize)]
struct PurchaseForm {
    ids : String,
    amounts : String
} 

#[web::put("/purchase")]
pub async fn create_purchase<'a>(request : web::HttpRequest, query : web::types::Form<PurchaseForm>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let mut collection : Vec<(i32, i32)> = Vec::new();

    let articles : Vec<i32> = query.ids.split(":").map(|id| id.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let amounts : Vec<i32> = query.amounts.split(":").map(|amount| amount.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    for x in 0..articles.len()-1 {
        collection.push((articles[x], amounts[x]));
    }

    let id : i32 = purchase::new_id(connection);

    purchase::create(&Purchase { id,
        id_user: 0,
        total: 0.0,
        time: Cow::Borrowed(&chrono::Local::now().to_string()),
        status: 0 }
    , connection);

    for (id_product, amount) in collection {
        let article : Article = Article { 
            id: article::new_id(connection),
            id_stock: id_product,
            id_purchase: id,
            amount
        };

        article::create(&article, connection);
    }

    return web::HttpResponse::Ok().body(Redirect{location: "/purchase/view/"}.render().unwrap()); 
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
