use csml_engine::{get_bot_by_version_id };
use crate::format_response;

use crate::{routes::GetByIdRequest};
use lambda_runtime::error::HandlerError;

pub fn handler(body: GetByIdRequest) -> Result<serde_json::Value, HandlerError> {

  let res = get_bot_by_version_id(&body.id, &body.bot_id);

  match res {
    Ok(data) => {
      match data {
        Some(data) => {
          Ok(serde_json::json!(
            {
              "isBase64Encoded": false,
              "statusCode": 200,
              "headers": { "Content-Type": "application/json" },
              "body": data
            }
          ))
        }
        None => {
          Ok(serde_json::json!({
            "isBase64Encoded": false,
            "statusCode": 400,
            "headers": { "Content-Type": "application/json" },
            "body": "Not found"
          }))
        }
      }
    },
    Err(err) => {
        let error = format!("EngineError: {:?}", err);
        return Ok(format_response(400, serde_json::json!(error)))
    }
  }
}