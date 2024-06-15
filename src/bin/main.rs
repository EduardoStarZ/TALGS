use TALGS::index;
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let adress : &str = "127.0.0.1";
    let port : u16 = 8080;

    web::HttpServer::new(|| {
        web::App::new()
            .service(index)
    })
    .bind((adress, port))?
    .run()
    .await
}
