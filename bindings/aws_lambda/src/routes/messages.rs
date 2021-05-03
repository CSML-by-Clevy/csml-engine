
use csml_engine::{Client};

use crate::{format_response};

use lambda_runtime::error::HandlerError;

pub fn get_client_messages(
    client: Client,
    limit: Option<i64>,
    pagination_key: Option<String>,
) -> Result<serde_json::Value, HandlerError> {

    let res = csml_engine::get_client_messages(
        &client, limit, pagination_key
    );

    match res {
        Ok(messages) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": messages
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)))
        }
    }
}