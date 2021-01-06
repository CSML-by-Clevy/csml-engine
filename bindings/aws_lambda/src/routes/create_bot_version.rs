use csml_engine::{create_bot_version};
use csml_interpreter::data::csml_bot::CsmlBot;
use crate::format_response;

use lambda_runtime::error::HandlerError;

pub fn handler(bot: CsmlBot) -> Result<serde_json::Value, HandlerError> {

  let res = create_bot_version(bot);

  match res {
    Ok(data) => {
        Ok(serde_json::json!(
        {
          "isBase64Encoded": false,
          "statusCode": 200,
          "headers": { "Content-Type": "application/json" },
          "body": data,
        }
      ))
    },
    Err(err) => {
        let error = format!("EngineError: {:?}", err);
        return Ok(format_response(400, serde_json::json!(error)))
    }
  }
}