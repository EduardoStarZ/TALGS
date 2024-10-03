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
use ntex::util::BytesMut;
use crate::colors::color::Color;
use askama::Template;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
struct FileForm {
    path: String,
}

#[web::post("/files")]
pub async fn file_receiver(request : web::HttpRequest, mut payload : web::types::Payload) -> web::HttpResponse{
    reqwestify(request);

    let mut bytes = BytesMut::new();
        while let Some(item) = ntex::util::stream_recv(&mut payload).await {
        bytes.extend_from_slice(&item.unwrap());
    }

    let path : String = String::from_utf8(bytes.into_iter().collect::<Vec<u8>>()).unwrap().split_once("path=").unwrap().1.to_string();

    let pure_bytes : Vec<u8> = crate::auth::parser::unspaced_hex_str_to_u8_vec(&path);

    println!("{}", path);
    let mut file : File = File::create("aarch.png").unwrap();
    file.write(&(*pure_bytes)).unwrap();

    return web::HttpResponse::Ok().finish();
}
