use csml_engine::{delete_client_memory, delete_client_memories, Client};

use crate::{format_response};

use lambda_runtime::error::HandlerError;

pub fn delete_memory(body: Client, memory_key: &str) -> Result<serde_json::Value, HandlerError> {

    let res = delete_client_memory(&body, memory_key);

    match res {
        Ok(_) => Ok(serde_json::json!(
            {
                "statusCode": 204,
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)))
        }
    }
}

pub fn delete_memories(body: Client) -> Result<serde_json::Value, HandlerError> {

    let res = delete_client_memories(&body);

    match res {
        Ok(_) => Ok(serde_json::json!(
            {
                "statusCode": 204,
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)))
        }
    }
}