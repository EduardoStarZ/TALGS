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

pub fn encrypt(data : &String, pub_key: &RsaPublicKey,rng: &mut ThreadRng) -> Vec<u8> {
    let byte_str = data.as_bytes();

    let enc_data : Vec<u8> = pub_key.encrypt(rng, Pkcs1v15Encrypt, byte_str).unwrap();


    return enc_data;
}

pub fn decrypt(enc_data : &Vec<u8>, priv_key : &RsaPrivateKey) -> String {
    let byte_data = &enc_data[..];
    let decoded_data : Vec<u8> = priv_key.decrypt(Pkcs1v15Encrypt, &byte_data).unwrap();

    return std::str::from_utf8(&decoded_data[..]).unwrap().to_string();
}
