use rand::{Rng, distributions::Alphanumeric};

pub fn create_hash() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    return s;
}
