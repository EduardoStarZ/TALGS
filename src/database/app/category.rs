use std::borrow::Cow;
use diesel::prelude::*;
use crate::schema::app::category;
use super::super::models::ResultCode;
use rand::{thread_rng, Rng};
use crate::colors::color::Color;
use diesel::SqliteConnection;
use serde::Deserialize;

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug, Deserialize)]
#[diesel(table_name = category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category<'a> {
    pub id: i16,
    pub name: Cow<'a, str>
}

pub fn create<'a, 'b>(category : &'a Category , connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&category.id, connection) {
        Some(value) => return Some(value),
        None => ()
    }

    match diesel::insert_into(category::table)
        .values(category)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn update<'a, 'b>(category : &'a Category, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&category.id, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(category::table)
        .filter(category::id.eq(category.id))
        .set(category)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn delete<'a, 'b>(id: &'a i16, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(id ,connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::delete(category::table)
        .filter(category::id.eq(id))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn exists<'a, 'b>(id: &'a i16, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    let q_keys : Vec<Category> = match category::table
        .filter(category::id.eq(id))
        .select(Category::as_select())
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

pub fn get<'a, 'b>(id: &'a i16, connection: &'b mut SqliteConnection) -> Option<Category<'a>> {
    let mut keys : Vec<Category<'a>> = match category::table
        .filter(category::id.eq(id))
        .select(Category::as_select())
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

pub fn new_id<'a>(connection : &'a mut SqliteConnection) -> i16 {
    let new : i16 = thread_rng().gen::<i16>();

     let keys : Vec<Category> = match category::table
        .filter(category::id.eq(new))
        .select(Category::as_select())
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

pub fn get_all<'a, 'b>(connection : &'b mut SqliteConnection) -> Vec<Category<'a>> {
    match category::table
        .select(Category::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return vec![];
            }
        }
}
