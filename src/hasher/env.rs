use std::env;

fn find_hash() -> Option<String> {
    for (key, value) in env::args() {
        if key == "hash" {
            return Some(value);
        } 
    }

    return None;
}
