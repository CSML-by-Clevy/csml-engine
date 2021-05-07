use crate::{format_response};

use lambda_runtime::error::HandlerError;

pub fn delete_bot(body: &str) -> Result<serde_json::Value, HandlerError> {

    let res = csml_engine::delete_all_bot_data(&body);

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