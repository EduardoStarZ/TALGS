use TALGS::{index, form};
use TALGS::database::connection::create_connection_to_auth;
use TALGS::database::models::{get_all_users, Crud, User};
use TALGS::session::controller::{login_handler, get_info_handler};
use ntex::web::{self, middleware::Logger};
use ntex_session::CookieSession;

#[web::get("/register/{group_id}/{user_name}")]
pub async fn register(path : web::types::Path<(i32, String)>) -> web::HttpResponse {
    

    let (group_id, username) = path.into_inner();

    let new_user : User = User {id: 0, name : username.clone(), group: group_id.clone(), cpf: "wololol".to_string(), password: "heheheha".to_string()};
    new_user.create(&mut create_connection_to_auth());

    let users : Vec<User> = get_all_users(&mut create_connection_to_auth());

    return web::HttpResponse::Ok().body(format!("New user created with name {} and with a group id of {}\n {:?}", &username, &group_id, &users[..]));
}

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
            .service(register)
    })
    .bind((adress, port))?
    .run()
    .await
}
