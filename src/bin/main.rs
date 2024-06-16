use TALGS::{index, form};
use TALGS::controller::{login_handler, get_info_handler};
use ntex::web::{self, middleware::Logger};
use ntex_session::CookieSession;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let adress : &str = "127.0.0.1";
    let port : u16 = 8080;

    web::HttpServer::new(|| {
        web::App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(index)
            .service(login_handler)
            .service(get_info_handler)
            .service(form)
    })
    .bind((adress, port))?
    .run()
    .await
}
