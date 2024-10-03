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

use std::{fs::File, io::Write};

use ntex::web;
use crate::colors::color::Color;
use askama::Template;
use crate::files::receiver;

pub mod auth;
pub mod app;


pub fn reqwestify(request: web::HttpRequest) {
    let headers : String = request.headers().iter().map(|x| format!("\t{:?} : {:?}\n", x.0, x.1)).collect::<String>();

    let reqwestified : String = format!("Request Type: {:?} | URI: {:?}\n Request Headers: \n{headers}\n", request.method(), request.uri());

    println!("{}", reqwestified.request());
}

#[derive(Template)]
#[template(path="files.html")]
struct FileFormTemplate {
}

#[web::get("/files")]
pub async fn file_sender(request : web::HttpRequest) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().body(FileFormTemplate{}.render().unwrap());
}

#[web::post("/files")]
pub async fn file_receiver(request : web::HttpRequest, payload : web::types::Payload) -> web::HttpResponse{
    reqwestify(request);

    let body : String = match receiver::read_payload_to_string(payload).await {
        Some(value) => value.split_once("path=").unwrap().1.to_string(),
        None => return web::HttpResponse::NotAcceptable().finish()
    };

    let pure_bytes : Vec<u8> = crate::auth::parser::unspaced_hex_str_to_u8_vec(&body);

    println!("{}", body);
    let mut file : File = File::create("aarch.png").unwrap();
    file.write(&(*pure_bytes)).unwrap();

    return web::HttpResponse::Ok().finish();
}
