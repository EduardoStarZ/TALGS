use diesel::prelude::*;
use crate::schema::app::article;
use super::super::models::ResultCode;
use rand::{thread_rng, Rng};
use crate::colors::color::Color;
use serde::Deserialize;

#[derive(Insertable, Selectable, Queryable, AsChangeset, Deserialize, Debug)]
#[diesel(table_name = article)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Article {
    pub id: i32,
    pub id_stock: i32,
    pub id_purchase: i32,
    pub amount: i32
}

pub fn create<'a, 'b>(article : &'a Article , connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&article.id, connection) {
        Some(value) => return Some(value),
        None => () 
    }

    match diesel::insert_into(article::table)
        .values(article)
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn update<'a, 'b>(article: &'a Article, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    match exists(&article.id, connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        },
        None => return Some(ResultCode::ValueError)
    }

    match diesel::update(article::table)
        .filter(article::id.eq(article.id))
        .set(article)
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

    match diesel::delete(article::table)
        .filter(article::id.eq(id))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                Some(ResultCode::ConnectionError)
            }
        }
}

pub fn exists<'a, 'b>(id: &'a i32, connection : &'b mut SqliteConnection) -> Option<ResultCode> {
    let q_keys : Vec<Article> = match article::table
        .filter(article::id.eq(id))
        .select(Article::as_select())
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

pub fn get<'a, 'b>(id: &'a i32, connection: &'b mut SqliteConnection) -> Option<Article> {
    let mut keys : Vec<Article> = match article::table
        .filter(article::id.eq(id))
        .select(Article::as_select())
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

     let keys : Vec<Article> = match article::table
        .filter(article::id.eq(new))
        .select(Article::as_select())
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

pub fn get_all<'a, 'b>(connection : &'b mut SqliteConnection) -> Vec<Article> {
    match article::table
        .select(Article::as_select())
        .load(connection) {
            Ok(value) => value,
            Err(err) => {
                println!("Error with the database: {}", err.to_string().warning());
                return vec![];
            }
        }
}
