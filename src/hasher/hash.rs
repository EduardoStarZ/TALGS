use rand::{thread_rng, Rng};

pub fn create_hash() -> String {
    let mut bytes : Vec<u8> = 0;
    
    for _ in 0..256 {
        bytes.push(thread_rng().gen::<u8>());
    }

    String::from_utf8(bytes).unwrap()
}
