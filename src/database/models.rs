/*
 *
 * models.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use serde::Serialize;

/// A Enum defined to give a better error reporting on the implementations of the CRUD struct
#[derive(Debug, Serialize)]
pub enum ResultCode {
    ConnectionError,
    ValueError,
    Exists,
    UnauthorizedError,
    InternalServerError
}
