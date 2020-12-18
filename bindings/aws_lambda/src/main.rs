mod routes;

use routes::{
    run, validate, RunRequest, GetByIdRequest, GetVersionsRequest , sns,
    create_bot_version, get_last_bot_version, get_bot_versions, get_bot_by_version_id,
    conversations::{close_user_conversations, get_open}
};

use csml_engine::Client;
use csml_interpreter::data::csml_bot::CsmlBot;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub fn format_response(status_code: i32, body: serde_json::Value) -> serde_json::Value {
    serde_json::json!(
        {
            "isBase64Encoded": false,
            "statusCode": status_code,
            "headers": { "Content-Type": "application/json" },
            "body": body
        }
    )
}

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
    path_parameters: Option<serde_json::Value>,
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
        } if path.ends_with("/run") && http_method == "POST" => {
            let body: RunRequest = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            run::handler(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/conversations/open") && http_method == "POST" => {
            let body: Client = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            get_open(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/conversations/close") && http_method == "POST" => {
            let body: Client =  match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            close_user_conversations(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/validate") && http_method == "POST" => {
            let body: CsmlBot = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            validate::handler(body)
        }

        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/create_bot_version") && http_method == "POST" => {
            let body: CsmlBot = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            create_bot_version::handler(body)
        }

        // GetByIdRequest 

        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/get_bot_by_version_id") && http_method == "POST" => {
            let body: GetByIdRequest = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            get_bot_by_version_id::handler(body)
        }

        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/get_last_bot_version") && http_method == "POST" => {
            let body: String = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            get_last_bot_version::handler(body)
        }

        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/get_bot_versions") && http_method == "POST" => {
            let body: GetVersionsRequest = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            get_bot_versions::handler(body)
        }

        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            headers,
            ..
        } if path.ends_with("/sns") && http_method == "POST" => {

            Ok(sns::handler(headers, body))
        }
        _ => Ok(format_response(404, serde_json::json!("Bad request")))
    }
}
