extern crate rusoto_s3;

use crate::EngineError;
use crate::data::DynamoDbClient;
use std::{io::Read};
use rusoto_s3::{S3, GetObjectRequest, DeleteObjectRequest, PutObjectRequest};

pub fn put_object(db: &mut DynamoDbClient, key: &str, content: String) -> Result<(), EngineError> {
    let bucket  = match std::env::var("AWS_S3_BUCKET") {
        Ok(bucket) => bucket,
        Err(_) => return Err(EngineError::Manager("Missing AWS_S3_BUCKET env var".to_owned())),
    };

    let request = PutObjectRequest{
        bucket,
        key: key.to_owned(),
        content_type: Some("application/json".to_owned()),
        body: Some(content.into_bytes().into()),
        ..Default::default()
    };

    let future = db.s3_client.put_object(request);

    let _value = db.runtime.block_on(future)?;

    Ok(())
}

pub fn get_object(db: &mut DynamoDbClient, key: &str) -> Result<String, EngineError> {
    let bucket  = match std::env::var("AWS_S3_BUCKET") {
        Ok(bucket) => bucket,
        Err(_) => return Err(EngineError::Manager("Missing AWS_S3_BUCKET env var".to_owned())),
    };

    let request =  GetObjectRequest{
        bucket,
        key: key.to_owned(),
        ..Default::default()
    };

    let future = db.s3_client.get_object(request);

    let value = db.runtime.block_on(future)?;

    match value.body{
        Some(value) => {
            let mut value = value.into_blocking_read();
            let mut buffer = String::new();

            value.read_to_string(&mut buffer)?;

            Ok(buffer)
        },
        None => return Err(EngineError::Manager("empty object".to_owned())),
    }
}

pub fn delete_object(db: &mut DynamoDbClient, key: &str) -> Result<(), EngineError> {
    let bucket  = match std::env::var("AWS_S3_BUCKET") {
        Ok(bucket) => bucket,
        Err(_) => return Err(EngineError::Manager("Missing AWS_S3_BUCKET env var".to_owned())),
    };
    
    let request =  DeleteObjectRequest{
        bucket,
        key: key.to_owned(),
        ..Default::default()
    };

    let future = db.s3_client.delete_object(request);

    db.runtime.block_on(future)?;

    Ok(())
}
