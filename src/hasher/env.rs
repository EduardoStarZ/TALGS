use std::fs;
use crate::colors::color::Color;

pub fn set_hash(value: String) {
    let mut contents : String = String::from_utf8(fs::read(".env").unwrap()).unwrap();
    contents.insert_str(0, format!("SESSION_SECRET={value}\n").as_str());
    
    match fs::write(".env", contents) {
        Ok(_) => (),
        Err(_) => panic!("{}", "The program could not set up the secret session string as a environment variable, shutting down...".warning())
    } 
}

pub fn get_hash() -> Option<String> {
    let contents : Vec<String> = String::from_utf8(fs::read(".env").unwrap()).unwrap().split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
    
    for line in contents {
        let pairs : Vec<String> = line.split('=').map(|x| x.to_string()).collect::<Vec<String>>();

        if pairs[0] == "SESSION_SECRET" {
            return Some(pairs[1].clone());
        }
    }
    return None;
}
