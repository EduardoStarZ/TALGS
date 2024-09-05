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

use std::ops::Deref;

use diesel::SqliteConnection;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation, decode};
use ntex::web;
use rand::thread_rng;
use crate::{auth::{encryption::{self, KeyPair}, parser}, colors::color::Color, database::{keys::{self, Keys}, models::ResultCode, users::{self, User}}, hasher, session::model::{Claims, LoginInfo, RegisterForm}};
use ntex_session::Session;
use super::model::LoginResponse;

///This is a handler that for any GET request to "/auth/login" that validates the information
///provided by an user and uses it to create a valid JWT from the secret in the .env file in case
///the information is fine, and can either return a LoginResponse Struct or a diesel Error
pub fn login_handler(login_info : &web::types::Form<LoginInfo>, auth_connection: &mut SqliteConnection, key_connection : &mut SqliteConnection) -> Result<LoginResponse, diesel::result::Error> {

    let email : &String = &login_info.email;

    let password : &String = &login_info.password;

    if is_valid(email, password, auth_connection, key_connection) {
        let claims : Claims = Claims { sub : email.clone(), exp: (chrono::Utc::now() + chrono::Duration::hours(3)).timestamp() as usize};
        let mut token : String = match encode(&Header::default(), &claims, &EncodingKey::from_secret(hasher::get_hash_in_env().as_str().as_ref())) {                               
            
            Ok(token) => token,
            Err(err) => { 
                println!("Error generating token: {}", err.to_string().warning());
                return Ok(LoginResponse { token: None, result: Some(ResultCode::ValueError)});
            }
        };

        token.insert_str(0, "Bearer ");

        return Ok(LoginResponse{token: Some(token), result: None});
    };
    return Ok(LoginResponse {token: None, result: Some(ResultCode::UnauthorizedError)});
}

#[web::get("/info")]
pub async fn get_info_handler(session : Session, request : web::HttpRequest) -> web::HttpResponse {
    println!("{request:?}");

    let auth_header : String = match session.get::<String>("Auth-Token").unwrap() {
        Some(value) => value,
        None => {
            return web::HttpResponse::Unauthorized().body("You are not authorized to see this page.") 
        }
    };

            if auth_header.starts_with("Bearer ") {
               let token : String = auth_header.trim_start_matches("Bearer ").to_string(); 
                
               match decode::<Claims>(&token, &DecodingKey::from_secret(hasher::get_hash_in_env().as_str().as_ref()), &Validation::default()) {
                    Ok(value) => {
                        let info : String = format!("You are valid, here is the information:\n email: {}\n TTL: {}", value.claims.sub, value.claims.exp);
                        return web::HttpResponse::Ok().body(info);
                    },
                    Err(err) => {
                        println!("Error generating token: {}", err.to_string().warning());
                        return web::HttpResponse::Unauthorized().finish();
                    }
               }
            }

    return web::HttpResponse::Unauthorized().finish();
}


///This one function is used by the above to validate the information
///the user provided, returns a simple boolean according to the success
fn is_valid(email: &str, password : &str, auth_conn: &mut SqliteConnection, key_conn : &mut SqliteConnection) -> bool {
    let user : User = match users::get(&email.to_string(), auth_conn) {
        Some(value) => value,
        None => return false
    };

    let keys : Keys = match keys::get(&user.id, key_conn) {
        Some(value) => value,
        None => return false
    };

    let encoded_priv_key = match encryption::str_to_private_key(&keys.private_key) {
        Some(value) => value,
        None => return false
    };
    
    let dec_password : String = match encryption::decrypt(&parser::unspaced_hex_str_to_u8_vec(&user.password), &encoded_priv_key) {
        Some(value) => value,
        None => return false
    };       

    dec_password == password
}


///This function is a handler function for any GET request to "/auth/register" that determines if
///the given email is not already present in the database, and can return a bool that indicates if
///the system was able to create the RSA keys or a diesel error if there is an account with the
///presented email
pub fn register_handler(form: web::types::Form<RegisterForm>, auth_conn: &mut SqliteConnection, key_conn : &mut SqliteConnection) -> Result<bool, diesel::result::Error> {
    let keys : KeyPair = match encryption::create_keys(1024) {
        Some(value) => value,
        None => return Ok(false)
    };

    let hashed_password = match encryption::encrypt(&form.password1, &keys.public_key, &mut thread_rng()) {
         Some(value) => value,
        None => return Ok(false)
    };

    let user : User = User {id: users::new_id(auth_conn), name: (&form.username).deref().to_string(), email: (&form.email).deref().to_string(), password: parser::unspaced_u8_vec_to_hex_str(&hashed_password), group: 1};

    match users::create(&user, auth_conn) {
        Some(_) => return Ok(false),
        None => (),
    };

    let keypair : Keys = Keys {id: keys::new_id(key_conn), user_id: user.id, public_key: encryption::public_key_to_str(&keys.public_key).unwrap(), private_key: encryption::private_key_to_str(&keys.private_key).unwrap() };

    match keys::create(&keypair, key_conn) {
        Some(_) => Ok(false),
        None => Ok(true)
    }
}
