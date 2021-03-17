use crate::data::{ast::Interval, position::Position};
use crate::error_format::*;

pub fn get_hash_algorithm(algo: &str, interval: Interval) -> Result<openssl::hash::MessageDigest, ErrorInfo> {
    match algo {
        "md5" => Ok(openssl::hash::MessageDigest::md5()),
        "sha1" => Ok(openssl::hash::MessageDigest::sha1()),
        "sha256" => Ok(openssl::hash::MessageDigest::sha256()),
        "sha384" => Ok(openssl::hash::MessageDigest::sha384()),
        "sha512" => Ok(openssl::hash::MessageDigest::sha512()),
        _ => Err(gen_error_info(
            Position::new(interval),
            format!("'{}' {}", algo, ERROR_HASH_ALGO),
        ))
    }
}

pub fn digest_data(algo: &str, data: &[u8], interval: Interval) -> Result<String, ErrorInfo> {
    match algo {
        "hex" => Ok(hex::encode(&data)),
        "base64" => Ok(openssl::base64::encode_block(&data)),
        _ => Err(gen_error_info(
            Position::new(interval),
            format!("'{}' {}", algo, ERROR_DIGEST_ALGO),
        ))
    }
}