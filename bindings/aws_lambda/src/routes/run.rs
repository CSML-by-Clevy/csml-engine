use csml_engine::start_conversation;
use serde_json::{json, Value};

use lambda_runtime::{error::HandlerError};

use crate::{LambdaResponse, routes::RunRequest};

pub fn handler(body: RunRequest) -> Result<LambdaResponse, HandlerError> {
    let bot = body.bot.to_owned();
    let mut request = body.event.to_owned();

    // request metadata should be an empty object by default
    request.metadata = match request.metadata {
        Value::Null => json!({}),
        val => val,
    };

    let res = start_conversation(request, bot);

    match res {
        Ok(data) => Ok(LambdaResponse {
            lambda_request: serde_json::json!(data),
        }),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            eprintln!("{}", error);
            Err(HandlerError::from(error.as_str()))
        }
    }
}
