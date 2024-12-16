use diesel::prelude::*;
use crate::schema::app::purchase;
use super::super::models::ResultCode;
use rand::{thread_rng, Rng};
use crate::colors::color::Color;
use chrono::NaiveDateTime;

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = purchase)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Purchase {
    pub id: i32,
    pub id_user: i32,
    pub time: NaiveDateTime, 
    pub status: i16
}

pub fn create<'a, 'b>(purchase : &'a Purchase , connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&purchase.id, connection) {
        Some(value) => return Some(value),
        None => ()
    }

    match diesel::insert_into(purchase::table)
        .values(purchase)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn update<'a, 'b>(purchase : &'a Purchase, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&purchase.id, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(purchase::table)
        .filter(purchase::id.eq(purchase.id))
        .set(purchase)
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

    match diesel::delete(purchase::table)
        .filter(purchase::id.eq(id))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn exists<'a, 'b>(id: &'a i32, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    let q_keys : Vec<Purchase> = match purchase::table
        .filter(purchase::id.eq(id))
        .select(Purchase::as_select())
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

pub fn get<'a, 'b>(id: &'a i32, connection: &'b mut SqliteConnection) -> Option<Purchase> {
    let mut keys : Vec<Purchase> = match purchase::table
        .filter(purchase::id.eq(id))
        .select(Purchase::as_select())
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

     let keys : Vec<Purchase> = match purchase::table
        .filter(purchase::id.eq(new))
        .select(Purchase::as_select())
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

pub fn get_all<'a, 'b>(connection : &'b mut SqliteConnection) -> Vec<Purchase> {
    match purchase::table
        .select(Purchase::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return vec![];
            }
        }
}
