/*
 *
 * model.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use serde::{Serialize, Deserialize};
use crate::database::models::ResultCode;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub email : String,
    pub password : String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token : Option<String>,
    pub result: Option<ResultCode>
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub : String,
    pub exp : usize
}
