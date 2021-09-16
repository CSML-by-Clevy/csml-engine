use crate::{format_response};

use lambda_runtime::error::HandlerError;

pub fn delete_expired_data() -> Result<serde_json::Value, HandlerError> {

    match csml_engine::delete_expired_data() {
        Ok(_) => Ok(serde_json::json!(
            {
                "statusCode": 200,
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)))
        }
    }
}
