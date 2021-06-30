use crate::data::{ast::Interval, position::Position};
use crate::error_format::*;

pub fn get_hash_algorithm(
    algo: &str,
    flow_name: &str,
    interval: Interval,
) -> Result<openssl::hash::MessageDigest, ErrorInfo> {
    match algo {
        "md5" | "MD5" => Ok(openssl::hash::MessageDigest::md5()),
        "sha1" | "SHA1" => Ok(openssl::hash::MessageDigest::sha1()),
        "sha256" | "SHA256" => Ok(openssl::hash::MessageDigest::sha256()),
        "sha384" | "SHA384" => Ok(openssl::hash::MessageDigest::sha384()),
        "sha512" | "SHA512" => Ok(openssl::hash::MessageDigest::sha512()),

        "sha3_224" | "SHA3_224" => Ok(openssl::hash::MessageDigest::sha3_224()),
        "sha3_256" | "SHA3_256" => Ok(openssl::hash::MessageDigest::sha3_256()),
        "sha3_384" | "SHA3_384" => Ok(openssl::hash::MessageDigest::sha3_384()),
        "sha3_512" | "SHA3_512" => Ok(openssl::hash::MessageDigest::sha3_512()),

        "shake_128" | "SHAKE_128" => Ok(openssl::hash::MessageDigest::shake_128()),
        "shake_256" | "SHAKE_256" => Ok(openssl::hash::MessageDigest::shake_256()),

        "ripemd160" | "RIPEMD160" => Ok(openssl::hash::MessageDigest::ripemd160()),
        "sm3" | "SM3" => Ok(openssl::hash::MessageDigest::sm3()),

        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            format!("'{}' {}", algo, ERROR_HASH_ALGO),
        )),
    }
}

pub fn digest_data(
    algo: &str,
    data: &[u8],
    flow_name: &str,
    interval: Interval,
) -> Result<String, ErrorInfo> {
    match algo {
        "hex" => Ok(hex::encode(&data)),
        "base64" => Ok(openssl::base64::encode_block(&data)),
        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            format!("'{}' {}", algo, ERROR_DIGEST_ALGO),
        )),
    }
}
