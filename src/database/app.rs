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
use crate::schema::app::{
    purchase};

pub mod article;
pub mod address;
pub mod product;
pub mod supplier;
pub mod category;
//pub mod purchase;
pub mod stock;

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = purchase)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Purchase {
    pub id: i32,
    pub id_user: i32,
    pub time: NaiveDateTime, 
    pub status: i16
}
