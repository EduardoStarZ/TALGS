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
