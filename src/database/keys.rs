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
use super::models::Crud;
use super::models::ResultCode;


///A struct defined to allow for CRUD implementations of the keys table
#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = key)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Keys {
    pub id: i32,
    pub user_id: i32,
    pub public_key: String,
    pub private_key: String
}

impl Crud for Keys {
    fn create(&self , connection : &mut SqliteConnection) -> Option<ResultCode> {
        match self.exists(connection) {
            Some(value) => return Some(value),
            None => ()
        }

        match diesel::insert_into(key::table)
            .values(self)
            .execute(connection) {
                Ok(_) => None,
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                    Some(ResultCode::ConnectionError)
                }
            }
    }

    fn update(&self , connection : &mut SqliteConnection) -> Option<ResultCode> {
        match self.exists(connection) {
            Some(value) => {
                match value {
                    ResultCode::Exists => (),
                    _ => return Some(value)
                }
            },
            None => return Some(ResultCode::ValueError)
        }
        
        match diesel::update(key::table)
           .filter(key::id.eq(self.id))
           .set(self)
           .execute(connection) {
                Ok(_) => None,
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                    Some(ResultCode::ConnectionError)
                }
           }
    }

    fn delete(&self, connection : &mut SqliteConnection) -> Option<ResultCode> {
        match self.exists(connection) {
            Some(value) => {
                match value {
                    ResultCode::Exists => (),
                    _ => return Some(value)
                }
            },
            None => return Some(ResultCode::ValueError)
        }

        match diesel::delete(key::table)
            .filter(key::id.eq(self.id))
            .execute(connection) {
                Ok(_) => None,
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                    Some(ResultCode::ConnectionError)
                }
            }
    }

    fn exists(&self, connection : &mut SqliteConnection) -> Option<ResultCode> {
        let q_keys : Vec<Keys> = match key::table
            .filter(key::id.eq(self.id))
            .select(Keys::as_select())
            .load(connection) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                    return Some(ResultCode::ConnectionError);
                }
            };

        match q_keys.is_empty() {
            true => None,
            false => Some(ResultCode::Exists)
        }
    }
}
