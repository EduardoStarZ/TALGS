/*
 *
 * hash.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use rand::{Rng, distributions::Alphanumeric};

///This function creates a 64 characters long string randomly
pub fn create_hash() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    return s;
}
