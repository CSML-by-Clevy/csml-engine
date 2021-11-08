use csml_interpreter::data::csml_bot::CsmlBot;
use crate::{format_response};

use lambda_runtime::error::HandlerError;

pub fn delete_bot_data(body: &str) -> Result<serde_json::Value, HandlerError> {

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

pub fn fold_bot(bot: CsmlBot) -> Result<serde_json::Value, HandlerError> {
    let res = csml_engine::fold_bot(bot);

    match res {
      Ok(flow) => {
          Ok(serde_json::json!(
          {
            "isBase64Encoded": false,
            "statusCode": 200,
            "headers": { "Content-Type": "application/json" },
            "body": {
                "flow": flow
            },
          }
        ))
      },
      Err(err) => {
          let error = format!("EngineError: {:?}", err);
          return Ok(format_response(400, serde_json::json!(error)))
      }
    }
  }