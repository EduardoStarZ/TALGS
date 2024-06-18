pub mod auth;
pub mod session;
pub mod database;
pub mod schema;


use ntex::web;
use askama::Template;

#[derive(Template)]
#[template(path="form.html")]
struct FormTemplate{}

#[web::get("/")]
pub async fn index() -> web::HttpResponse {
    return web::HttpResponse::Ok().body("Hello world");
}

#[web::get("/form")]
pub async fn form() -> web::HttpResponse {
    return web::HttpResponse::Ok().body(FormTemplate{}.render().unwrap())
}
