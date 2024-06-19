/*
 *
 * encryption.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use rand::rngs::ThreadRng;
use rsa::{pkcs1::
        {DecodeRsaPrivateKey, DecodeRsaPublicKey,
        EncodeRsaPrivateKey, EncodeRsaPublicKey},
        pkcs8::LineEnding, Pkcs1v15Encrypt,
        RsaPrivateKey, RsaPublicKey};


///A struct that encapsulates a set of an RSA public and private keys
pub struct KeyPair {
    pub public_key : RsaPublicKey,
    pub private_key : RsaPrivateKey
}


///Function that takes a amount of bits to build up a new KeyPair struct,
///
///May return a None value if the function cannot create a key
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


///Function that takes up a String reference, a public key reference and 
///a thread rng reference to build up a vector of bits
///
///This function may return None if the given string is long enough to overflow the bits given to
///during keys creation
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


//Function that takes up a reference to a vector of bits and a reference to a private key
//to decode the message
//
//This function may return None if the message was encripted with a key that was not
//originated by the given private key
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


///This function takes up a reference to a public key and serializes it to PEM encoding
///to produce a String
///
///May return a None value
pub fn public_key_to_str(public_key : &RsaPublicKey) -> Option<String> {
    match public_key.to_pkcs1_pem(LineEnding::default()) {
       Ok(value) => Some(value),
       Err(err) => {
            eprintln!("Error while serializing public key to PEM encoding: {err}");
            None
       }
    }
}


///This function takes up a reference to a PEM encoded public key as a string and
///deserialize it back to a proper RSA public key
///
///May return a None value if the given String is not a proper encoded key
pub fn str_to_public_key(public_key : &String) -> Option<RsaPublicKey> {
    match RsaPublicKey::from_pkcs1_pem(public_key.as_str()) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("Error while deserializing public key from PEM encoding: {err}");
            None
        }
    }
}


///This function takes up a reference to a private key and serializes to PEM encoding
///to produce a String
///
///May return a None value if the given Key is not in the correct format
pub fn private_key_to_str(private_key: &RsaPrivateKey) -> Option<String> {
    match private_key.to_pkcs1_pem(LineEnding::default()) {
        Ok(value) => Some(value.to_string()),
        Err(err) => {
            eprintln!("Error while serializing private key to PEM encoding: {err}");
            None
        }
    }
}


///This function takes up a reference to a PEM encoded public key as a String and 
///deserializes it back to a proper RSA private key
///
///May return a None value if the given key is not a proper encoded key
pub fn str_to_private_key(private_key: &String) -> Option<RsaPrivateKey> {
    match RsaPrivateKey::from_pkcs1_pem(private_key.as_str()) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("Error while deserializing private key from PEM encoding: {err}");
            None
        }
    }
}
