/**
 * Encryption and decryption utils for securely storing sensitive data.
 * It is not mandatory to setup encryption in CSML engine, however it is greatly recommended
 * if the chatbot is handling sensitive data of any sort.
 *
 * To automatically setup encryption/decryption of data, you must set an ENCRYPTION_SECRET
 * environment variable with a complex enough string.
 *
 * Encrypt: Data is JSON-stringified before encryption, and is returned as an encrypted string.
 * Decrypt: Data is decrypted from an encrypted string and is returned as a JSON Value.
 *
 * The encryption algorithm used is AES-256-GCM.
 */

use crate::ManagerError;

use openssl::{
    pkcs5::pbkdf2_hmac,
    rand::rand_bytes,
    symm::{decrypt_aead, encrypt_aead, Cipher},
};
use std::env;

fn get_key(salt: &[u8], key: &mut [u8]) -> Result<(), ManagerError> {
    let pass = match env::var("ENCRYPTION_SECRET") {
        Ok(var) => var,
        _ => panic!("No ENCRYPTION_SECRET value in env"),
    };

    pbkdf2_hmac(
        pass.as_bytes(),
        &salt,
        10000,
        openssl::hash::MessageDigest::sha512(),
        key,
    )?;

    Ok(())
}

fn encrypt(text: &[u8]) -> Result<String, ManagerError> {
    let cipher = Cipher::aes_256_gcm();

    let mut tag = vec![0; 16];
    let mut iv = vec![0; 16];
    rand_bytes(&mut iv)?;
    let mut salt = vec![0; 64];
    rand_bytes(&mut salt)?;
    let mut key = [0; 32];
    get_key(&salt, &mut key)?;

    let encrypted = encrypt_aead(cipher, &key, Some(&iv), &[], text, &mut tag)?;

    Ok(base64::encode(&[salt, iv, tag, encrypted].concat()))
}

pub fn encrypt_data(value: &serde_json::Value) -> Result<String, ManagerError> {
    match env::var("ENCRYPTION_SECRET") {
        Ok(..) => encrypt(&value.to_string().as_bytes()),
        _ => Ok(value.to_string()),
    }
}

fn decrypt(text: String) -> Result<String, ManagerError> {
    let ciphertext = base64::decode(text)?;
    let cipher = Cipher::aes_256_gcm();

    let iv_length = 16;
    let salt_length = 64;
    let tag_length = 16;
    let tag_position = salt_length + iv_length;
    let encrypted_position = tag_position + tag_length;

    let salt: &[u8] = &ciphertext[0..salt_length];
    let iv: &[u8] = &ciphertext[salt_length..tag_position];
    let tag: &[u8] = &ciphertext[tag_position..encrypted_position];
    let encrypted: &[u8] = &ciphertext[encrypted_position..];

    let mut key = [0; 32];
    get_key(&salt, &mut key)?;

    let value = decrypt_aead(cipher, &key, Some(&iv), &[], &encrypted, &tag)?;

    Ok(String::from_utf8_lossy(&value).to_string())
}

pub fn decrypt_data(value: String) -> Result<serde_json::Value, ManagerError> {
    match env::var("ENCRYPTION_SECRET") {
        Ok(..) => {
            let value: serde_json::Value = serde_json::from_str(&decrypt(value)?)?;
            Ok(value)
        }
        _ => {
            let value: serde_json::Value = serde_json::from_str(&value)?;
            Ok(value)
        }
    }
}
