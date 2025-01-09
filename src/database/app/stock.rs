use diesel::prelude::*;
use crate::schema::app::stock;
use super::super::models::ResultCode;
use rand::{thread_rng, Rng};
use crate::colors::color::Color;
use std::borrow::Cow;
use serde::Deserialize;

#[derive(Insertable, Selectable, Queryable, AsChangeset, Deserialize, Debug)]
#[diesel(table_name = stock)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Stock<'a> {
    pub id: i32,
    pub id_product: i32,
    pub id_supplier: i32,
    pub expired: bool,
    pub expire_date: Cow<'a, str>,
    pub available: bool,
    pub batch: Option<i64>,
    pub amount: i32
}

pub fn create<'a, 'b>(stock : &'a Stock , connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&stock.id, connection) {
        Some(value) => return Some(value),
        None => ()
    }

    match diesel::insert_into(stock::table)
        .values(stock)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn update<'a, 'b>(stock : &'a Stock, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&stock.id, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(stock::table)
        .filter(stock::id.eq(stock.id))
        .set(stock)
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

    match diesel::delete(stock::table)
        .filter(stock::id.eq(id))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn exists<'a, 'b>(id: &'a i32, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    let q_keys : Vec<Stock> = match stock::table
        .filter(stock::id.eq(id))
        .select(Stock::as_select())
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

pub fn get<'a, 'b>(id: &'a i32, connection: &'b mut SqliteConnection) -> Option<Stock<'a>> {
    let mut keys : Vec<Stock> = match stock::table
        .filter(stock::id.eq(id))
        .select(Stock::as_select())
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

     let keys : Vec<Stock> = match stock::table
        .filter(stock::id.eq(new))
        .select(Stock::as_select())
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

pub fn get_all<'a, 'b>(connection : &'b mut SqliteConnection) -> Vec<Stock<'a>> {
    match stock::table
        .select(Stock::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return vec![];
            }
        }
}
