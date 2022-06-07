use csml_engine::{
    delete_client_memories, delete_client_memory, get_client_memories, get_client_memory, Client,
};

use crate::{format_response, Error};

pub fn delete_memory(body: Client, memory_key: &str) -> Result<serde_json::Value, Error> {
    let res = delete_client_memory(&body, memory_key);

    match res {
        Ok(_) => Ok(serde_json::json!(
            {
                "statusCode": 204,
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)));
        }
    }
}

pub fn delete_memories(body: Client) -> Result<serde_json::Value, Error> {
    let res = delete_client_memories(&body);

    match res {
        Ok(_) => Ok(serde_json::json!(
            {
                "statusCode": 204,
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)));
        }
    }
}

pub fn get_memory(body: Client, key: &str) -> Result<serde_json::Value, Error> {
    let res = get_client_memory(&body, key);

    match res {
        Ok(value) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": value
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)));
        }
    }
}

pub fn get_memories(body: Client) -> Result<serde_json::Value, Error> {
    let res = get_client_memories(&body);

    match res {
        Ok(value) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": value
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)));
        }
    }
}

pub fn create_client_memory(
    client: Client,
    key: String,
    value: serde_json::Value,
) -> Result<serde_json::Value, Error> {
    let res = csml_engine::create_client_memory(&client, key, value);

    match res {
        Ok(_) => Ok(serde_json::json!(
            {
                "statusCode": 201,
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)));
        }
    }
}
