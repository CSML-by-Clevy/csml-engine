use csml_engine::{start_conversation, data::BotOpt};
use serde_json::{json, Value};
use crate::format_response;

use lambda_runtime::{error::HandlerError};

use crate::{routes::RunRequest};

pub fn handler(body: RunRequest) -> Result<serde_json::Value, HandlerError> {
    let bot = body.bot.to_owned();
    let mut request = body.event.to_owned();

    // request metadata should be an empty object by default
    request.metadata = match request.metadata {
        Value::Null => json!({}),
        val => val,
    };

    let res = start_conversation(request, BotOpt::CsmlBot(bot));

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
