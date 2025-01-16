use ntex::web;
use ntex_session::Session;
use askama::Template;
use crate::database::connection::{AuthPool, AppPool};
use super::super::views::reqwestify;

//#[derive(Template)]
//#[template(path="")]
//struct NewAdressTemplate {}
//
//#[derive(Template)]
//#[template(path="")]
//struct UpdateAdressTemplate {}
//
//#[derive(Template)]
//#[template(path="")]
//struct DeleteAdressTemplate {}

pub async fn create<T : Template>(request : web::HttpRequest, session : Session, app_pool : web::types::State<AppPool>, auth_pool : web::types::State<AuthPool>, template : T) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().body(template.render().unwrap());
}
    

pub async fn update<T: Template>(request : web::HttpRequest, session : Session, app_pool : web::types::State<AppPool>, auth_pool : web::types::State<AuthPool>, template : T) -> web::HttpResponse {
    reqwestify(request);   

    return web::HttpResponse::Ok().body(template.render().unwrap());
}
    

pub async fn delete<T : Template>(request : web::HttpRequest, session : Session, app_pool : web::types::State<AppPool>, auth_pool : web::types::State<AuthPool>, template : T) -> web::HttpResponse {
    reqwestify(request);

    return web::HttpResponse::Ok().body(template.render().unwrap());
}
