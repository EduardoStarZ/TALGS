/*
 *
 * users.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use diesel::prelude::*;
use rand::{thread_rng, Rng};
use crate::{colors::color::Color, schema::auth::users};
use super::models::ResultCode;


///A struct defined for CRUD implementations of the users table
#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id : i32,
    pub name : String,
    pub password : String,
    pub group : i32,
    pub email : String
}

pub fn create(user : &User, connection : &mut SqliteConnection) -> Option<ResultCode> { 
    match exists(&user.email, connection) {
        Some(value) => return Some(value),
        None => (),
    }

    match diesel::insert_into(users::table)
        .values(user)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return Some(ResultCode::ConnectionError);
            }
        }
}

pub fn exists(email: &String, connection : &mut SqliteConnection) -> Option<ResultCode> {
    let q_users : Vec<User> = match users::table
        .filter(users::email.eq(email))
        .select(User::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Error with the database: {}", err.to_string().warning());
                return Some(ResultCode::ConnectionError);
            }
        };

    return match q_users.is_empty() {
        true => None,
        false => Some(ResultCode::Exists)
    }

}

pub fn update(user : &User, connection : &mut SqliteConnection) -> Option<ResultCode> {
    match exists(&user.email, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(users::table)
        .filter(users::id.eq(user.id))
        .set(user)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}


pub fn delete(user : &User, connection : &mut SqliteConnection) -> Option<ResultCode> {
    match exists(&user.email, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        }
        None => return Some(ResultCode::ValueError)
    }

    match diesel::delete((users::table).filter(users::id.eq(&user.id)))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database : {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            } 
        }
}

pub fn get(email : &String, connection : &mut SqliteConnection) -> Option<User> {
    match exists(&email, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                    _ => return None
            }
        },
        None => return None
    }

    let mut users : Vec<User> = match users::table
        .filter(users::email.eq(email))
        .select(User::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return None;
            }
        };

    users.reverse();

    return users.pop();
}

pub fn new_id(auth_conn : &mut SqliteConnection) -> i32 {
    let new : i32 = thread_rng().gen::<i32>();

     let users : Vec<User> = match users::table
        .filter(users::id.eq(new))
        .select(User::as_select())
        .load(auth_conn) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return new_id(auth_conn);
            }
        };

     if !users.is_empty() {
        return new_id(auth_conn);
     }

     return new;
}