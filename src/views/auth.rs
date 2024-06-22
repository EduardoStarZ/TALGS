use ntex::web;
use r2d2::ManageConnection;
use serde::Deserialize;
use super::super::database::{keys::Keys, users::User};
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate{}

#[web::get("/login")]
pub async fn login() -> web::HttpResponse {
    return web::HttpResponse::Ok().body(LoginTemplate{}.render().unwrap());
}
