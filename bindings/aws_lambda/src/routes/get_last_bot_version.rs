use csml_engine::{get_last_bot_version};
use crate::format_response;

use lambda_runtime::error::HandlerError;

pub fn handler(bot_id: String) -> Result<serde_json::Value, HandlerError> {

  let res = get_last_bot_version(&bot_id);

  match res {
    Ok(data) => {
      match data {
        Some(data) => {
          Ok(serde_json::json!(
            {
              "isBase64Encoded": false,
              "statusCode": 200,
              "headers": { "Content-Type": "application/json" },
              "body": data.flatten()
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