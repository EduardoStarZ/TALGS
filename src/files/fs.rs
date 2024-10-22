/*
 *
 * fs.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use std::fs;
use rand::{Rng, distributions::Alphanumeric};
use crate::colors::color::Color;

pub enum FileExt {
    PNG,
    SVG,
    JPEG
}

pub struct Image {
    pub name : String,
    pub content : Vec<u8>,
}

pub trait Constructor {
    fn new() -> Image;
}

impl Constructor for Image {
    fn new() -> Image {
        return Image{ name: String::new(), content: Vec::new()};
    }
}

pub fn detect_file_extension(file : Image) -> Option<FileExt> {
    let ext : Vec<String> = file.name.split(".").map(|x| x.to_string()).collect::<Vec<String>>();

    return match ext[ext.len()-1].as_str() {
        "png" => Some(FileExt::PNG),
        "svg" => Some(FileExt::SVG),
        "jpg" | "jpeg" => Some(FileExt::JPEG),
        _ => None
    }
}

pub fn write_contents(bytes : Vec<u8>, filename : &String) -> bool {
    if exists(&filename) {
        return false;
    }

    return match fs::write(format!("/img/{filename}").as_str(), bytes) {
        Ok(_) => true,
        Err(err) => {
            println!("{}", err.to_string().warning());
            false
        }
    };

}

fn exists(path : & String) -> bool {
    return match fs::read(path) {
        Ok(_) => true,
        Err(err) => {
            println!("{}", err.to_string().warning());
            return false;
        }
    }
}

pub fn create_file(filename : String) -> bool {
    if exists(&filename) {
        return false;
    }

    return match fs::File::create(format!("/img/{filename}").as_str()) {
        Ok(_) => true,
        Err(err) => {
            println!("{}", err.to_string().warning());
            false
        }
    };
}

pub fn check_dir_existance() -> bool {
    let path = fs::read_dir("/static/app/img/");
    
    return match path {
        Ok(_) => true,
        Err(err) => {
            println!("{}", err.to_string().warning());
            false
        }
    }
}

pub fn create_dir() {
    match fs::create_dir_all("/static/app/img/") {
        Ok(_) => return,
        Err(err) => {
            println!("{}", err.to_string().warning());
        }
    };
}

pub fn rand_name() -> String {
     let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect::<String>();

    return s;
   
}
