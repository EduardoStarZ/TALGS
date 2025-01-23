/*
 *
 * views.rs
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
use crate::{colors::color::Color};
use askama::Template;

pub mod auth;
pub mod app;


pub fn reqwestify(request: web::HttpRequest) {
    let headers : String = request.headers().iter().map(|x| format!("\t{:?} : {:?}\n", x.0, x.1)).collect::<String>();

    let reqwestified : String = format!("Request Type: {:?} | URI: {:?}\n Request Headers: \n{headers}\n", request.method(), request.uri());

    println!("{}", reqwestified.request());
}

pub fn transform_payload(body : String) -> Vec<String> {
    return body.split("&").map(|x| x.to_string()).collect::<Vec<String>>();
}

#[derive(Deserialize)]
struct TestStruct {
    field : String
}

#[derive(Template)]
#[template( path = "test.html")]
struct TestTemplate {}

#[web::get("/test")]
pub async fn get_test_route(request : web::HttpRequest, query: web::types::Query<TestStruct>) -> web::HttpResponse {
    reqwestify(request);

    println!("{}", (*query).field);

    return web::HttpResponse::Ok().body(TestTemplate{}.render().unwrap());
}

#[derive(Deserialize)]
struct TestForm {
    field : String
}

#[web::post("/test")]
pub async fn post_test_route(request : web::HttpRequest, form : web::types::Form<TestForm>) -> web::HttpResponse {
    reqwestify(request);

    //let strng : String = read_payload_to_string(payload).await.unwrap();

    println!("{}", (*form).field);
    

    return web::HttpResponse::Ok().body("Status code 200: Ok");
}
