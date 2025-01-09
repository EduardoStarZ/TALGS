use diesel::prelude::*;
use serde::Deserialize;
use crate::schema::app::product;
use super::super::models::ResultCode;
use rand::{thread_rng, Rng};
use crate::colors::color::Color;
use std::borrow::Cow;

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug, Deserialize)]
#[diesel(table_name = product)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Product<'a> {
    pub id: i32,
    pub name: Cow<'a, str>,
    pub image: Cow<'a, str>,
    pub price: f32,
    pub warn_at: i32,
    pub id_category: i16,
    pub total_amount: i32,
    pub measure: i32,
    pub measure_unit: i16
}

pub fn create<'a, 'b>(product : &'a Product , connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&product.id, connection) {
        Some(value) => return Some(value),
        None => ()
    }

    match diesel::insert_into(product::table)
        .values(product)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn update<'a, 'b>(product : &'a Product, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&product.id, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(product::table)
        .filter(product::id.eq(product.id))
        .set(product)
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

    match diesel::delete(product::table)
        .filter(product::id.eq(id))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn exists<'a, 'b>(id: &'a i32, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    let q_keys : Vec<Product> = match product::table
        .filter(product::id.eq(id))
        .select(Product::as_select())
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

pub fn get<'a, 'b>(id: &'a i32, connection: &'b mut SqliteConnection) -> Option<Product<'a>> {
    let mut keys : Vec<Product<'a>> = match product::table
        .filter(product::id.eq(id))
        .select(Product::as_select())
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

     let keys : Vec<Product> = match product::table
        .filter(product::id.eq(new))
        .select(Product::as_select())
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

pub fn get_all<'a, 'b>(connection : &'b mut SqliteConnection) -> Vec<Product<'a>> {
    match product::table
        .select(Product::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return vec![];
            }
        }
}
