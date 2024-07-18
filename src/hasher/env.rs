use std::{env, fs};
use std::fs::File;

pub fn find_hash() -> Option<String> {
    for (key, value) in env::args() {
        if key == "hash" {
            return Some(value);
        } 
    }

    return None;
}

pub fn set_hash(value: String) -> Result<()> {
    let mut contents : String = String::from_utf8(fs::read(".env").unwrap()).unwrap();
    contents.insert_str(0, format!("hash={value}\n").as_str());
    fs::write(".env", contents) 
}


