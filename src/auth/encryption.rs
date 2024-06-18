use rand::rngs::ThreadRng;
use rsa::{pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey}, pkcs8::LineEnding, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub struct KeyPair {
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

    return Some(KeyPair { public_key: pub_key, private_key: priv_key})                            
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

pub fn public_key_to_str(public_key : &RsaPublicKey) -> Option<String> {
    match public_key.to_pkcs1_pem(LineEnding::default()) {
       Ok(value) => Some(value),
       Err(err) => {
            eprintln!("Error while serializing public key to PEM encoding: {err}");
            None
       }
    }
}

pub fn str_to_public_key(public_key : &String) -> Option<RsaPublicKey> {
    match RsaPublicKey::from_pkcs1_pem(public_key.as_str()) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("Error while deserializing public key from PEM encoding: {err}");
            None
        }
    }
}
