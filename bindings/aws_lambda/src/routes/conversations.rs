use csml_engine::{user_close_all_conversations, get_open_conversation, Client};

use lambda_runtime::error::HandlerError;
use crate::{LambdaResponse};


pub fn get_open(body: Client) -> Result<LambdaResponse, HandlerError> {

    let res = get_open_conversation(&body);

    match res {
        Ok(Some(conversation)) => Ok(LambdaResponse {
            lambda_request: serde_json::json!(conversation),
        }),
        Ok(None) => Ok(LambdaResponse {
            lambda_request: serde_json::json!(false),
        }),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            eprintln!("{}", error);
            Err(HandlerError::from(error.as_str()))
        }
    }
}

pub fn close_user_conversations(body: Client) -> Result<LambdaResponse, HandlerError> {

    let res = user_close_all_conversations(body.clone());

    match res {
        Ok(()) => Ok(LambdaResponse {
            lambda_request: serde_json::json!(true),
        }),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            eprintln!("{}", error);
            Err(HandlerError::from(error.as_str()))
        }
    }
}
