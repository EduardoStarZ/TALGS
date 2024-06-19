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

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username : String,
    pub password : String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token : String
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub : String,
    pub exp : usize
}
