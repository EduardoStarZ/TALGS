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

use ntex::web;
use serde::Deserialize;
use crate::database::connection::AppPool;
use crate::database::app::product::{Product, self};
use super::super::reqwestify;
use diesel::prelude::*;
use crate::files::receiver::read_payload_to_string;
use crate::str::filter::{Form, payload_into_values};
use crate::files::fs::{self, rand_name};
use crate::auth::parser::unspaced_hex_str_to_u8_vec;
use std::borrow::Cow;

#[derive(Deserialize)]
struct Q {
    field : String
}

#[web::get("/product/{id}")]
pub async fn product_reader(request : web::HttpRequest, selected_id : web::types::Path<i32>, query : web::types::Query<Q>, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    match *query.field {
        _ => println!("{}", query.field)
    }

    return web::HttpResponse::Ok().body("");
}

#[web::put("/product")]
pub async fn create_product(request : web::HttpRequest, payload : web::types::Payload, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let payload : String = match read_payload_to_string(payload).await {
        Some(value) => value,
        None => return web::HttpResponse::BadRequest().finish()
    };

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    let values : Vec<Form> = payload_into_values(&payload);

    let mut product : Product = Product {
        id: product::new_id(connection),
        name: Cow::from(""),
        price: 0.0,
        id_category: 0,
        warn_at: 0,
        measure: 0,
        measure_unit: 0,
        image: Cow::from(""),
        total_amount: 0
    };

    let mut filename : String = String::new();

    for x in values {
        match x.name {
            "filename" => {
                filename = format!("{}.{}", rand_name(), x.value
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

                fs::create_file(&filename.clone());
            },
            "bytes" => {
                fs::write_contents(&unspaced_hex_str_to_u8_vec(&String::from(x.value)), &filename);
            },
            "name" => {
                product.name = Cow::from(x.value);
            },
            "price" => {
                product.price = x.value.trim().parse::<f32>().unwrap();
            },
            "category" => {
                product.id_category = x.value.trim().parse::<i16>().unwrap();
            },
            "warn_at" => {
                product.warn_at = x.value.trim().parse::<i32>().unwrap();
            },
            "measure" => {
                product.measure = x.value.trim().parse::<i32>().unwrap();
            },
            "measure_unit" => {
                product.measure_unit = x.value.trim().parse::<i16>().unwrap();
            },
            _ => return web::HttpResponse::Forbidden().finish()
        } 
    }

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

#[web::delete("/product")]
pub async fn delete_product(request : web::HttpRequest, form : web::types::Form<i32> , pool : web::types::State<AppPool>) -> web::HttpResponse {

    reqwestify(request);

    let connection : &mut SqliteConnection = &mut pool.pool.get().unwrap();

    product::delete(&*form, connection);
    
    return web::HttpResponse::Ok().body("delete");
}
