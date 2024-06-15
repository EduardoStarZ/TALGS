use ntex::web;
use askama::Template;
use serde::{Serialize, Deserialize};

#[web::get("/")]
pub async fn index() -> web::HttpResponse {
    return web::HttpResponse::Ok().body("Hello world");
}
