use csml_engine::{get_bot_versions };
use crate::format_response;

use lambda_runtime::error::HandlerError;
use crate::{routes::GetVersionsRequest};


pub fn handler(body: GetVersionsRequest) -> Result<serde_json::Value, HandlerError> {

  let res = get_bot_versions(&body.bot_id, body.limit, body.last_key);

  match res {
    Ok(data) => Ok(serde_json::json!(
      {
        "isBase64Encoded": false,
        "statusCode": 200,
        "headers": { "Content-Type": "application/json" },
        "body": data
      }
    )),
    Err(err) => {
        let error = format!("EngineError: {:?}", err);
        return Ok(format_response(400, serde_json::json!(error)))
    }
  }
}