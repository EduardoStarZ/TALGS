/*
 *
 * keys.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use diesel::prelude::*;
use crate::schema::key::key;
use super::models::ResultCode;
use rand::{thread_rng, Rng};
use crate::colors::color::Color;
use std::borrow::Cow;


///A struct defined to allow for CRUD implementations of the keys table
#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = key)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Keys<'a> {
    pub id: i32,
    pub user_id: i32,
    pub public_key: Cow<'a, str>,
    pub private_key: Cow<'a, str>
}

pub fn create<'a, 'b>(keys : &'a Keys , connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&keys.id, connection) {
        Some(value) => return Some(value),
        None => ()
    }

    match diesel::insert_into(key::table)
        .values(keys)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn update<'a, 'b>(keys : &'a Keys , connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&keys.id, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(key::table)
        .filter(key::id.eq(keys.id))
        .set(keys)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn delete<'a, 'b>(id: &'a i32, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(id ,connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::delete(key::table)
        .filter(key::id.eq(id))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn exists<'a, 'b>(id: &'a i32, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    let q_keys : Vec<Keys> = match key::table
        .filter(key::id.eq(id))
        .select(Keys::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return Some(ResultCode::ConnectionError);
            }
        };

    match q_keys.is_empty() {
        true => None,
        false => Some(ResultCode::Exists)
    }
}

pub fn get<'a, 'b>(id: &'a i32, connection: &'b mut SqliteConnection) -> Option<Keys<'a>> {
    let mut keys : Vec<Keys<'a>> = match key::table
        .filter(key::user_id.eq(id))
        .select(Keys::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return None;
            }
        };

    keys.reverse();

    return keys.pop();
}

pub fn new_id<'a>(key_conn : &'a mut SqliteConnection) -> i32 {
    let new : i32 = thread_rng().gen::<i32>();

     let keys : Vec<Keys> = match key::table
        .filter(key::id.eq(new))
        .select(Keys::as_select())
        .load(key_conn) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return new_id(key_conn);
            }
        };

     if !keys.is_empty() {
        return new_id(key_conn);
     }

     return new;
}

pub fn get_all<'a, 'b>(key_conn : &'b mut SqliteConnection) -> Vec<Keys<'a>> {
    match key::table
        .select(Keys::as_select())
        .load(key_conn) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return vec![];
            }
        }
}
