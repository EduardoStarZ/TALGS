/*
 *
 * controller.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use diesel::SqliteConnection;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation, decode};
use ntex::web;
use crate::{auth::{encryption, parser}, database::{keys::{self, Keys}, models::ResultCode, users::{self, User}}, session::model::{Claims, LoginInfo}};
use ntex_session::Session;
use super::model::LoginResponse;

pub fn login_handler(login_info : &web::types::Form<LoginInfo>, auth_connection: &mut SqliteConnection, key_connection : &mut SqliteConnection) -> LoginResponse {

    let email : &String = &login_info.email;

    let password : &String = &login_info.password;

    if is_valid(email, password, auth_connection, key_connection) {
        let claims : Claims = Claims { sub : email.clone(), exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize};
        let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {                               
            
            Ok(token) => token,
            Err(e) => { 
                eprintln!("Error generating token: {e}");
                return LoginResponse { token: None, result: Some(ResultCode::ValueError)};
            }
        };

        return LoginResponse{token: Some(token), result: None};
    };
    return LoginResponse {token: None, result: Some(ResultCode::UnauthorizedError)};
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

fn is_valid(email: &str, password : &str, auth_conn: &mut SqliteConnection, key_conn : &mut SqliteConnection) -> bool {
    let user : User = match users::get(&email.to_string(), auth_conn) {
        Some(value) => value,
        None => return false
    };

    let keys : Keys = match keys::get(&user.id, key_conn) {
        Some(value) => value,
        None => return false
    };

    let encoded_priv_key = encryption::str_to_private_key(&keys.private_key).unwrap();
    
    let dec_password : String = encryption::decrypt(&parser::unspaced_hex_str_to_u8_vec(&user.password), &encoded_priv_key).unwrap();       
    dec_password == password
}
