use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation, decode};
use ntex::web;
use crate::session::model::{Claims, LoginInfo, LoginResponse};
use ntex_session::Session;

#[web::post("/login")]
pub async fn login_handler(session : Session, login_info : web::types::Form<LoginInfo>, request : web::HttpRequest) -> web::HttpResponse {

    println!("{request:?}");

    let username : &String = &login_info.username;

    let password : &String = &login_info.password;

    if is_valid(username, password) {
        let claims : Claims = Claims { sub : username.clone(), exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize};
        let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {                               
            
            Ok(token) => token,
            Err(e) => { 
                eprintln!("Error generating token: {e}");
                return web::HttpResponse::InternalServerError().finish();
            }
        };

        session.set("Auth-Token", &token).unwrap();

        return web::HttpResponse::Ok().body(LoginResponse{token}.token);
    };
    return web::HttpResponse::Unauthorized().finish();
}

#[web::get("/info")]
pub async fn get_info_handler(session : Session, request : web::HttpRequest) -> web::HttpResponse {
    println!("{request:?}");

    let mut auth_header : String = session.get::<String>("Auth-Token").unwrap().unwrap();
    auth_header.insert_str(0, "Bearer ");

            if auth_header.starts_with("Bearer ") {
               let token : String = auth_header.trim_start_matches("Bearer ").to_string(); 
                
               match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
                    Ok(_) => {
                        let info : String = String::from("You are valid, here is the information");
                        return web::HttpResponse::Ok().body(info);
                    },
                    Err(e) => {
                        eprintln!("Error generating token: {e}");
                        return web::HttpResponse::Unauthorized().finish();
                    }
               }
            }

    return web::HttpResponse::Unauthorized().finish();
}

fn is_valid(username : &str, password : &str) -> bool {
    username != "" && password != ""
}
