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
use std::borrow::Cow;

//pub mod article;
//pub mod address;
//pub mod product;
//pub mod supplier;
//pub mod category;
//pub mod purchase;
//pub mod stock;

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
pub struct Supplier<'a> {
    pub id: i32,
    pub name: Cow<'a, str>,
    pub cnpj: Option<Cow<'a, str>>,
    pub cpf: Option<Cow<'a, str>>,
    pub email: Cow<'a, str>
}

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
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

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category<'a> {
    pub id: i16,
    pub name: Cow<'a, str>
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
