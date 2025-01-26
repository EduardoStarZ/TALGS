use diesel::SqliteConnection;
use ntex::web;
//use ntex_session::Session;
use askama::Template;
use crate::database::{app::purchase::{self, Purchase}, connection::AppPool};
use super::super::views::reqwestify;

struct PurchaseRender {
    pub id: i32,
    pub time: String,
    pub date: String,
    pub total: f32,
    pub status_str: String
}

#[derive(Template)]
#[template(path="sale/client.html")]
struct NewPurchaseTemplate{
    purchases : Vec<PurchaseRender>
}

#[web::get("/purchase/new/")]
pub async fn create(request : web::HttpRequest, app_pool : web::types::State<AppPool> ) -> web::HttpResponse {
    reqwestify(request);

    let connection : &mut SqliteConnection = &mut app_pool.pool.get().unwrap();

    let purchases_struct : Vec<Purchase> = purchase::get_all(connection);

    let purchases : Vec<PurchaseRender> = purchases_struct
        .iter()
        .map(|purchase| PurchaseRender{
            id : purchase.id,
            date: purchase.time.split("T").next().unwrap().replace("-", "/").to_string(),

            time: purchase.time.split("T").nth(1).unwrap().to_string(),
            total: purchase.total,
            status_str: match purchase.status {
                0 => String::from("Em andamento"),
                1 => String::from("Pronto"),
                2 => String::from("Compra Realizada"),
                _ => String::from("Indefinido")
            }
        }).collect::<Vec<PurchaseRender>>();

    return web::HttpResponse::Ok().body(NewPurchaseTemplate{purchases}.render().unwrap());
}
