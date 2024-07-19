use std::{env, fs};
use crate::colors::color::Color;

pub fn find_hash() -> Option<String> {
    for (key, value) in env::vars() {
        if key == "HASH" {
            return Some(value);
        } 
    }

    return None;
}

pub fn set_hash(value: String) {
    let mut contents : String = String::from_utf8(fs::read(".env").unwrap()).unwrap();
    contents.insert_str(0, format!("HASH={value}\n").as_str());
    
    match fs::write(".env", contents) {
        Ok(_) => (),
        Err(_) => panic!("{}", "The program could not set up the hash for the environment variable, shutting down...".warning())
    } 
}

pub fn get_hash() -> Option<String> {
    for (key, value) in env::vars() {
        if key == "HASH" {
            return Some(value);
        }
    }

    return None;
}
