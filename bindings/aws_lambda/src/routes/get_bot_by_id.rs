use csml_engine::{get_bot_by_id };
use crate::format_response;

use crate::{routes::GetByIdRequest};
use lambda_runtime::error::HandlerError;

pub fn handler(body: GetByIdRequest) -> Result<serde_json::Value, HandlerError> {

  let res = get_bot_by_id(&body.id, &body.bot_id);

  match res {
    Ok(data) => Ok(serde_json::json!(
      {
        "isBase64Encoded": false,
        "statusCode": 200,
        "headers": { "Content-Type": "application/json" },
        "body": serde_json::json!(data).to_string()
      }
    )),
    Err(err) => {
        let error = format!("EngineError: {:?}", err);
        return Ok(format_response(400, serde_json::json!(error)))
    }
  }
}