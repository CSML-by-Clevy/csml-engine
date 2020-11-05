mod routes;

use routes::{
    run, validate, RunRequest,
    conversations::{close_user_conversations, get_open}
};

use csml_engine::Client;
use csml_interpreter::data::csml_bot::CsmlBot;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(lambda_handler);
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LambdaRequest {
    resource: String,
    path: String,
    http_method: String,
    request_context: serde_json::Value,
    headers: serde_json::Value,
    multi_value_headers: serde_json::Value,
    query_string_parameters: Option<serde_json::Value>,
    multi_value_query_string_parameters: Option<serde_json::Value>,
    path_parameters: Option<serde_json::Value,
    stage_variables: Option<serde_json::Value>,
    body: Option<String>,
    is_base64_encoded: bool,
}


fn lambda_handler(request: LambdaRequest, _c: Context) -> Result<serde_json::Value, HandlerError> {
    match request {
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path == "/run" && http_method == "POST" => {
            let body: RunRequest = serde_json::from_str(&body).unwrap();

            run::handler(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path == "/conversations/open" && http_method == "POST" => {
            let body: Client = serde_json::from_str(&body).unwrap();

            get_open(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path == "/conversations/close" && http_method == "POST" => {
            let body: Client = serde_json::from_str(&body).unwrap();

            close_user_conversations(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path == "/validate" && http_method == "POST" => {
            let body: CsmlBot = serde_json::from_str(&body).unwrap();

            validate::handler(body)
        }
        _ => Err(HandlerError::from("Bad request")),
    }
}
