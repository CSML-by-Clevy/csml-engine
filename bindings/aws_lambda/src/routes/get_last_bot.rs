use csml_engine::{get_last_bot_version};
use csml_interpreter::data::csml_bot::CsmlBot;
use crate::format_response;

use lambda_runtime::error::HandlerError;

pub fn handler(bot_id: String) -> Result<serde_json::Value, HandlerError> {

  let res = get_last_bot_version(bot_id);

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
        return format_response(400, serde_json::json!(error))
    }
  }
}