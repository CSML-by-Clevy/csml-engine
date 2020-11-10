mod routes;

use routes::{
    run, validate, RunRequest, sns,
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
            let body: RunRequest = serde_json::from_str(&body).unwrap();

            run::handler(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/conversations/open") && http_method == "POST" => {
            let body: Client = serde_json::from_str(&body).unwrap();

            get_open(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/conversations/close") && http_method == "POST" => {
            let body: Client = serde_json::from_str(&body).unwrap();

            close_user_conversations(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/validate") && http_method == "POST" => {
            let body: CsmlBot = serde_json::from_str(&body).unwrap();

            validate::handler(body)
        }
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            headers,
            ..
        } if path.ends_with("/sns") && http_method == "POST" => {
            // let body: String = serde_json::from_str(&body).unwrap();
            // let body_string = match std::str::from_utf8(&body) {
            //     Ok(res) => res,
            //     Err(_) => return HttpResponse::BadRequest().body("Request body can not be properly parsed"),
            // };

            Ok(sns::handler(headers, body))
        }
        _ => Ok(format_response(404, serde_json::json!("Bad request")))
    }
}
