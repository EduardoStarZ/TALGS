/*
 *
 * product.rs
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
use crate::database::connection::AppPool;
use crate::database::app::product::{Product, self};
use super::super::reqwestify;
use diesel::prelude::*;
use crate::files::fs::{self, rand_name};
use crate::auth::parser::unspaced_hex_str_to_u8_vec;
use std::borrow::Cow;


#[derive(Deserialize)]
struct ProductQuery {
    category: Option<String>,
    exclude: Option<String>,
    only: Option<i32>

}

#[derive(Template)]
#[template(path = "product/available_card.html")]
struct AvailableProductCard<'a> {
    products : Vec<Product<'a>>
}

#[derive(Template)]
#[template(path = "product/selected_card.html")]
struct SelectedProductCard<'a> {
    product : Product<'a>
}

#[web::get("/product/{format}")]
pub async fn product_reader(request : web::HttpRequest, query : web::types::Query<ProductQuery>, path : web::types::Path<String>, app_pool : web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut app_pool.pool.get().unwrap();

    let mut products : Vec<Product> = product::get_all(connection);

    match &query.category {
        Some(value) => {
            println!("{value}");
            if value.as_str() != "none" {
                products = products.into_iter().filter(|product| product.id_category == value.parse::<i16>().unwrap()).collect();
            }
        },
        None => ()
    }

    match &query.exclude {
        Some(value) => {
            println!("{value}");
            let ids : Vec<i32> = value.split(":").map(|id| id.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            for id in ids {
                products = products.into_iter().filter(|product| product.id != id).collect();
            }
        },
        None => ()
    }

    match path.as_str() {
            "available-card" => {
                match query.only {
                    Some(value) => return web::HttpResponse::Ok().body(AvailableProductCard{products: Vec::from(product::get(value).unwrap())}),
                    None => ()
                }

                return web::HttpResponse::Ok().body(AvailableProductCard{products}.render().unwrap());                
            },
            "selected-card" => {
                let id : i32 = query.only.unwrap();
                let product : Product = product::get(&id, connection).unwrap();
                return web::HttpResponse::Ok().body(SelectedProductCard{product}.render().unwrap());
            }
            _ => ()
    } 

    return web::HttpResponse::Ok().body("");
}

#[derive(Deserialize)]
struct ProductReceiver {
    name : String,
    price : f32,
    id_category : i16,
    warn_at: i32,
    measure: i32,
    measure_unit: i16,
    filename: String,
    bytes: String
}

#[web::put("/product/")]
pub async fn create_product(request : web::HttpRequest, form : web::types::Form<ProductReceiver>, pool: web::types::State<AppPool>) ->  web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let filename : String = format!("{}.{}", rand_name(), &form.filename
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


    let product : Product = Product {
        id: product::new_id(connection),
        image: Cow::Borrowed(&*filename),
        name: Cow::Borrowed(&*form.name),
        price: form.price,
        warn_at: form.warn_at,
        total_amount: 0,
        measure: form.measure,
        measure_unit: form.measure_unit,
        id_category: form.id_category 
    };

    fs::create_file(&filename.clone());
            
    fs::write_contents(&unspaced_hex_str_to_u8_vec(&form.bytes), &filename);
    
    product::create(&product, connection);

    return web::HttpResponse::Ok().finish(); 
}

#[web::patch("/product")]
pub async fn update_product<'a>(request : web::HttpRequest, form : web::types::Form<Product<'a>> , pool: web::types::State<AppPool>) -> web::HttpResponse {
    
    reqwestify(request);
    
    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();
    
    product::update(&*form, connection);

    return web::HttpResponse::Ok().body("update");
}

#[web::delete("/product/{id}")]
pub async fn delete_product(request : web::HttpRequest, id : web::types::Path<i32> ,  pool : web::types::State<AppPool>) -> web::HttpResponse {

    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    product::delete(&*id, connection);
    
    return web::HttpResponse::Ok().body("delete");
}
