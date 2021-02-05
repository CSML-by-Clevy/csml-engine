extern crate s3;

use crate::EngineError;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::S3Error;

fn get_region(region: &str) -> Result<Region, S3Error> {
    let region = region.parse::<Region>()?;

    Ok(region)
}

pub fn get_bucket() -> Result<Bucket, EngineError> {
    let region_name = match std::env::var("AWS_REGION") {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    let s3_endpoint = match std::env::var("AWS_S3_ENDPOINT") {
        Ok(val) => Some(val),
        Err(_) => None,
    };

    let region = match (region_name, s3_endpoint) {
        (Some(region_name), None) => get_region(&region_name)?,
        (Some(region_name), Some(s3_endpoint)) => Region::Custom {
            region: region_name,
            endpoint: s3_endpoint,
        },
        _ => return Err(EngineError::Manager("Invalid AWS_S3 environment setup".to_owned()))
    };

    let credentials = Credentials::default().unwrap();

    let bucket  = match std::env::var("AWS_S3_BUCKET") {
        Ok(bucket) => bucket,
        Err(_) => return Err(EngineError::Manager("Missing AWS_S3_BUCKET env var".to_owned())),
    };

    // Create Bucket in REGION
    Ok(Bucket::new(&bucket, region, credentials)?)
}

pub fn put_object(bucket: &Bucket, key: &str, content: &[u8]) -> Result<(), EngineError> {
    let (_, code) = bucket.put_object_with_content_type_blocking(key, content, "application/json")?;

    match code {
        code  if code == 200 => Ok(()),
        code => Err(EngineError::S3ErrorCode(code))
    }
}

pub fn get_object(bucket: &Bucket, key: &str) -> Result<Vec<u8>, EngineError> {
    let (value, code) = bucket.get_object_blocking(key)?;

    match code {
        code if code == 200 => Ok(value),
        code => Err(EngineError::S3ErrorCode(code))
    }
}

pub fn delete_object(bucket: &Bucket, key: &str) -> Result<(), EngineError> {
    let (_, code) = bucket.delete_object_blocking(key)?;

    match code {
        code if code == 204 => Ok(()),
        code => Err(EngineError::S3ErrorCode(code))
    }
}
