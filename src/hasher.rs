/*
 *
 * hasher.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

pub mod env;
pub mod hash;

use crate::colors::color::Color;

pub fn get_hash_in_env() -> String {
    let probable_hash : Option<String> = env::get_hash();
    
    match probable_hash {
        Some(value) => return value,
        None => {
            env::set_hash(hash::create_hash());
            return match env::get_hash() {
                Some(value) => value,
                None => panic!("{}", "The program could not create the hash for session encryption successfully, shutting down...".warning())
            }
        }
    };
}
