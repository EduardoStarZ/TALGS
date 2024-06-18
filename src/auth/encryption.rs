use rand::rngs::ThreadRng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub struct KeyPair {
    pub org_rng : ThreadRng,
    pub public_key : RsaPublicKey,
    pub private_key : RsaPrivateKey
}

pub fn create_keys(bits : usize) -> Option<KeyPair> {
    let mut rng : ThreadRng = rand::thread_rng();
    
    let priv_key_builder : Result<RsaPrivateKey, rsa::Error> = RsaPrivateKey::new(&mut rng, bits);                   

    let priv_key : RsaPrivateKey = match priv_key_builder {
        Ok(value) => {value},
        Err(_) => return None,
    };

    let pub_key : RsaPublicKey = RsaPublicKey::from(&priv_key);

    return Some(KeyPair { org_rng: rng, public_key: pub_key, private_key: priv_key})
}

pub fn encrypt(data : &String, pub_key: &RsaPublicKey,rng: &mut ThreadRng) -> Option<Vec<u8>> {
    let byte_str = data.as_bytes();

    let enc_data : Option<Vec<u8>> = match pub_key.encrypt(rng, Pkcs1v15Encrypt, byte_str) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("Error while encrypting data: {err}");
            None
        }
    };

    return enc_data;
}

pub fn decrypt(enc_data : &Vec<u8>, priv_key : &RsaPrivateKey) -> Option<String> {
    let byte_data = &enc_data[..];
    let decoded_data : Option<Vec<u8>> = match priv_key.decrypt(Pkcs1v15Encrypt, &byte_data) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("Error while decrypting data: {err}");
            return None;
        }
    };

    return  match std::str::from_utf8(&decoded_data.unwrap()[..]) {
        Ok(value) => Some(value.to_string()),
        Err(err) => {
            eprintln!("Error while parsing byte sequence to a utf-8 string: {err}");
            None
        }
    }
}
