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
use crate::colors::color::Color;
use peak_alloc::PeakAlloc;
use askama::Template;
use serde::Deserialize;

pub mod auth;
pub mod app;

static PEAK : PeakAlloc = PeakAlloc;

pub fn reqwestify(request: web::HttpRequest) {
    let headers : String = request.headers().iter().map(|x| format!("\t{:?} : {:?}\n", x.0, x.1)).collect::<String>();

    let reqwestified : String = format!("Request Type: {:?} | URI: {:?}\n Request Headers: \n{headers}\n", request.method(), request.uri());

    println!("{}", reqwestified.request());
    let usage : (String, String) = (format!("Current Memory Usage: {}", PEAK.current_usage_as_kb()), format!("Maximum Memory Usage: {}", PEAK.peak_usage_as_kb()));

    println!("{} | {}", usage.0.warning(), usage.1.warning());
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

#[derive(Deserialize)]
struct FileForm {
    path: String,
}

#[web::post("/files")]
pub async fn file_receiver(request : web::HttpRequest, form : web::types::Form<FileForm>) -> web::HttpResponse {
    reqwestify(request);

    println!("{}", form.path);

    return web::HttpResponse::Ok().finish();
}
