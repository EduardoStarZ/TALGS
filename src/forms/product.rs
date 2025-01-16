use diesel::SqliteConnection;
use ntex::web;
//use ntex_session::Session;
use askama::Template;
use crate::database::{app::{category::{self, Category}, product::{self, Product} }, connection::AppPool};
use super::super::views::reqwestify;

#[derive(Template)]
#[template(path="product/new.html")]
struct NewProductTemplate<'a> {
    categories : Vec<Category<'a>>
}

#[web::get("/product/new/")]
pub async fn create(request : web::HttpRequest, app_pool : web::types::State<AppPool> ) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut app_pool.pool.get().unwrap();

    let categories : Vec<Category> = category::get_all(connection);

    return web::HttpResponse::Ok().body(NewProductTemplate{categories}.render().unwrap());
}

#[derive(Template)]
#[template(path = "product/update.html")]
struct UpdateProductTemplate<'a> {
    product : Product<'a>
}

#[web::get("product/update/{id}")]
pub async fn update(request : web::HttpRequest, id : web::types::Path<i32>, app_pool : web::types::State<AppPool>) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut app_pool.pool.get().unwrap();

    let product : Product = product::get(&(*id), connection).unwrap();

    return web::HttpResponse::Ok().body(UpdateProductTemplate{product}.render().unwrap());
}
