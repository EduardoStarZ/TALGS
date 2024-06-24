use ntex::web;
use crate::database::connection::{AuthPool, KeyPool};
use serde::Deserialize;
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate{}

#[derive(Deserialize)]
struct LoginForm {
    pub email: String,
    pub password: String
}

#[web::get("/login")]
pub async fn login(auth_pool : web::types::State<AuthPool>, key_pool : web::types::State<KeyPool> ,form : web::types::Form<LoginForm>) -> web::HttpResponse {
    let mut auth_connection = match auth_pool.pool.get() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to receive a connection to the database: {err}");
            return web::HttpResponse::InternalServerError().body("");
        }
    };

    let mut key_connection = match key_pool.pool.get() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to receive a connection to the database: {err}");
            return web::HttpResponse::InternalServerError().body("")
        }
    };

    return web::HttpResponse::Ok().body(LoginTemplate{}.render().unwrap());
}
