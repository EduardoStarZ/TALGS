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

pub fn write_contents(bytes : Vec<u8>, filename : String) -> bool {
    if fs::exists(&filename).unwrap() {
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
