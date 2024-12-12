use std::borrow::Cow;
use diesel::prelude::*;
use crate::schema::app::supplier;
use super::super::models::ResultCode;
use rand::{thread_rng, Rng};
use crate::colors::color::Color;
use diesel::SqliteConnection;

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = supplier)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Supplier<'a> {
    pub id: i32,
    pub name: Cow<'a, str>,
    pub cnpj: Option<Cow<'a, str>>,
    pub cpf: Option<Cow<'a, str>>,
    pub email: Cow<'a, str>
}

pub fn create<'a, 'b>(supplier : &'a Supplier , connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&supplier.id, connection) {
        Some(value) => return Some(value),
        None => ()
    }

    match diesel::insert_into(supplier::table)
        .values(supplier)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn update<'a, 'b>(supplier : &'a Supplier, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&supplier.id, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(supplier::table)
        .filter(supplier::id.eq(supplier.id))
        .set(supplier)
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

    match diesel::delete(supplier::table)
        .filter(supplier::id.eq(id))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn exists<'a, 'b>(id: &'a i32, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    let q_keys : Vec<Supplier> = match supplier::table
        .filter(supplier::id.eq(id))
        .select(Supplier::as_select())
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

pub fn get<'a, 'b>(id: &'a i32, connection: &'b mut SqliteConnection) -> Option<Supplier<'a>> {
    let mut keys : Vec<Supplier<'a>> = match supplier::table
        .filter(supplier::id.eq(id))
        .select(Supplier::as_select())
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

     let keys : Vec<Supplier> = match supplier::table
        .filter(supplier::id.eq(new))
        .select(Supplier::as_select())
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
    match supplier::table
        .select(Supplier::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return vec![];
            }
        }
}
