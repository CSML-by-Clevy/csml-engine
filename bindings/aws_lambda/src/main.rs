mod routes;

use routes::{
    run, validate, GetVersionsRequest, BotIdVersionIdPath, BotIdPath,
    sns,
    bot_versions::{
        add_bot_version, get_bot_latest_version, get_bot_latest_versions, get_bot_version,
        delete_bot_versions, delete_bot_version

    },
    conversations::{close_user_conversations, get_open}
};

use csml_engine::{data::RunRequest, Client};
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
        } if path.ends_with("/bots") && http_method == "POST" => {
            let body: CsmlBot = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            add_bot_version(body)
        }

        LambdaRequest {
            path,
            http_method,
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/bots/{bot_id}/versions/{version_id}") && http_method == "GET" => {
            let path_parameters: BotIdVersionIdPath = match serde_json::from_value(path_params) {
                Ok(path_parameters) => path_parameters,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            get_bot_version(path_parameters)
        }

        LambdaRequest {
            path,
            http_method,
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/bots/{bot_id}") && http_method == "GET" => {
            let path_params: BotIdPath = match serde_json::from_value(path_params) {
                Ok(path_params) => path_params,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            get_bot_latest_version(path_params.bot_id)
        }

        LambdaRequest {
            path,
            http_method,
            query_string_parameters: Some(query_params),
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/bots/{bot_id}") && http_method == "GET" => {
            let path_params: BotIdPath = match serde_json::from_value(path_params) {
                Ok(path_params) => {
                    path_params
                },
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            let mut params = GetVersionsRequest{bot_id: path_params.bot_id, limit: None, pagination_key: None };

            if let Some(serde_json::Value::Number(limit))= query_params.get("limit") {
                params.limit = limit.as_i64();
            }

            if let Some(serde_json::Value::String(pagination_key)) = query_params.get("pagination_key") {
                params.pagination_key = Some(pagination_key.to_owned());
            }

            get_bot_latest_versions(params)
        }

        LambdaRequest {
            path,
            http_method,
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/bots/{bot_id}") && http_method == "DELETE" => {
            let path_params: BotIdPath = match serde_json::from_value(path_params) {
                Ok(path_params) => {
                    path_params
                },
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };
            delete_bot_versions(path_params.bot_id)
        }

        LambdaRequest {
            path,
            http_method,
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/bots/{bot_id}/versions/{version_id}") && http_method == "DELETE" => {
            let path_params: BotIdVersionIdPath = match serde_json::from_value(path_params) {
                Ok(path_params) => {
                    path_params
                },
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            delete_bot_version(path_params.bot_id, path_params.version_id)
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
