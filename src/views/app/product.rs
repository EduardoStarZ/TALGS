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
use askama::Template;
use crate::database::connection::AppPool;
use crate::database::app::{Product, Category};
use super::super::reqwestify;
use diesel::prelude::*;
use crate::schema::app::*;
use crate::files::receiver::read_payload_to_string;
use crate::str::filter::{Form, payload_into_values};
use crate::files::fs::{self, rand_name};
use crate::auth::parser::unspaced_hex_str_to_u8_vec;
use std::borrow::Cow;


#[derive(Template)]
#[template(path="new_product.html")]
struct NewProductpage<'a> {
    categories : Vec<Category<'a>>
}

#[web::get("/product/create")]
pub async fn create_product_route(request : web::HttpRequest, pool : web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let categories : Vec<Category> = category::table
        .select(Category::as_select())
        .load(&mut pool.pool.get().unwrap())
        .unwrap();

    return web::HttpResponse::Ok().body(NewProductpage{categories}.render().unwrap());
}

#[web::put("/product/create")]
pub async fn create_product_receiver(request : web::HttpRequest, payload : web::types::Payload, pool: web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let payload : String = match read_payload_to_string(payload).await {
        Some(value) => value,
        None => return web::HttpResponse::BadRequest().finish()
    };

    let values : Vec<Form> = payload_into_values(&payload);

    let mut product : Product = Product {
        id: 134536,
        name: Cow::from(""),
        price: 0.0,
        id_category: 0,
        warn_at: 0,
        measure: 12,
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

    diesel::insert_into(product::table)
        .values(&product)
        .execute(&mut pool.pool.get().unwrap())
        .unwrap();

    return web::HttpResponse::Ok().finish(); 
}
