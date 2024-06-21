use ntex::web;
use r2d2::ManageConnection;
use serde::Deserialize;
use super::super::database::{keys::Keys, users::User};
use askama::Template;

#[web::get("/login")]
pub async fn login() -> web::HttpResponse {

    return web::HttpResponse::Ok().body("test");
}
