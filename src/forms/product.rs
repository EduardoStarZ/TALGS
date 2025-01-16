use diesel::SqliteConnection;
use ntex::web;
use ntex_session::Session;
use askama::Template;
use crate::database::{app::category::{self, Category}, connection::{AppPool, AuthPool}};
use super::super::views::reqwestify;


#[derive(Template)]
#[template(path="new_product.html")]
struct NewProductTemplate<'a> {
    categories : Vec<Category<'a>>
}

#[web::get("/product/new/")]
pub async fn create(request : web::HttpRequest, session : Session, app_pool : web::types::State<AppPool>, auth_pool : web::types::State<AuthPool>) -> web::HttpResponse {
    let connection : &mut SqliteConnection = &mut app_pool.pool.get().unwrap();

    let categories : Vec<Category> = category::get_all(connection);

    reqwestify(request);

    return web::HttpResponse::Ok().body(NewProductTemplate{categories}.render().unwrap());
}
    

pub async fn update<T: Template>(request : web::HttpRequest, session : Session, app_pool : web::types::State<AppPool>, auth_pool : web::types::State<AuthPool>, template : T) -> web::HttpResponse {
    reqwestify(request);   

    return web::HttpResponse::Ok().body(template.render().unwrap());
}
    

pub async fn delete<T : Template>(request : web::HttpRequest, session : Session, app_pool : web::types::State<AppPool>, auth_pool : web::types::State<AuthPool>, template : T) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().body(template.render().unwrap());
}
