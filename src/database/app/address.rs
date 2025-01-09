use diesel::prelude::*;
use crate::schema::app::address;
use super::super::models::ResultCode;
use rand::{thread_rng, Rng};
use crate::colors::color::Color;
use std::borrow::Cow;
use serde::Deserialize;

#[derive(Insertable, Selectable, Queryable, AsChangeset,  Deserialize, Debug)]
#[diesel(table_name = address)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Address<'a> {
    pub id: i32,
    pub id_supplier: i32,
    pub cep: i32,
    pub city: Cow<'a, str>,
    pub neighborhood: Cow<'a, str>,
    pub block: Option<Cow<'a, str>>,
    pub number: Cow<'a, str>,
    pub complement: Option<Cow<'a, str>>
}

pub fn create<'a, 'b>(address : &'a Address, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&address.id, connection) {
        Some(value) => return Some(value),
        None => ()
    }

    match diesel::insert_into(address::table)
        .values(address)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn update<'a, 'b>(address: &'a Address, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&address.id, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(address::table)
        .filter(address::id.eq(address.id))
        .set(address)
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

    match diesel::delete(address::table)
        .filter(address::id.eq(id))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn exists<'a, 'b>(id: &'a i32, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    let q_keys : Vec<Address<'a>> = match address::table
        .filter(address::id.eq(id))
        .select(Address::as_select())
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

pub fn get<'a, 'b>(id: &'a i32, connection: &'b mut SqliteConnection) -> Option<Address<'a>> {
    let mut keys : Vec<Address<'a>> = match address::table
        .filter(address::id.eq(id))
        .select(Address::as_select())
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

pub fn new_id<'a>(connection : &'a mut SqliteConnection) -> i32 {
    let new : i32 = thread_rng().gen::<i32>();

     let keys : Vec<Address<'a>> = match address::table
        .filter(address::id.eq(new))
        .select(Address::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return new_id(connection);
            }
        };

     if !keys.is_empty() {
        return new_id(connection);
     }

     return new;
}

pub fn get_all<'a, 'b>(connection : &'b mut SqliteConnection) -> Vec<Address<'a>> {
    match address::table
        .select(Address::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return vec![];
            }
        }
}
