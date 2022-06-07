use crate::format_response;
use csml_engine::start_conversation;
use serde_json::{json, Value};

use crate::{routes::RunRequest, Error};

pub fn handler(body: RunRequest) -> Result<serde_json::Value, Error> {
    let mut request = body.event.to_owned();

    let bot_opt = match body.get_bot_opt() {
        Ok(bot_opt) => bot_opt,
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)));
        }
    };

    // request metadata should be an empty object by default
    request.metadata = match request.metadata {
        Value::Null => json!({}),
        val => val,
    };

    let res = start_conversation(request, bot_opt);

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
            return Ok(format_response(400, serde_json::json!(error)));
        }
    }
}
