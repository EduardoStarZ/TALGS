/*
 *
 * app.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::app::*;


#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = article)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Article {
    pub id: i32,
    pub id_stock: i32,
    pub id_purchase: i32,
    pub amount: i32
}

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = product)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub id_category: i16,
    pub total_amount: i32
}

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = purchase)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Purchase {
    pub id: i32,
    pub id_user: i32,
    pub time: NaiveDateTime, 
    pub status: i16
}

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = supplier)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Supplier {
    pub id: i32,
    pub name: String,
    pub cnpj: Option<String>,
    pub cpf: Option<String>,
    pub email: String
}

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = adress)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Adress {
    pub id: i32,
    pub id_supplier: i32,
    pub cep: i32,
    pub city: String,
    pub neighborhood: String,
    pub block: Option<String>,
    pub number: String,
    pub complement: Option<String>
}

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i16,
    pub name: String
}

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = stock)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Stock {
    pub id: i32,
    pub id_product: i32,
    pub id_supplier: i32,
    pub expired: bool,
    pub expire_date: NaiveDateTime,
    pub available: bool,
    pub batch: Option<i64>,
    pub amount: i32
}
