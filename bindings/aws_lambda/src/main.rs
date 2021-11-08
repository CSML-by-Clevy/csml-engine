mod routes;
mod helpers;

use routes::{
    BotIdPath, BotIdVersionIdPath, GetVersionsRequest,
    MemoryBody, MemoryKeyPath,
    bot_versions::{
        add_bot_version, get_bot_latest_version, get_bot_latest_versions, get_bot_version,
        delete_bot_versions, delete_bot_version
    },
    bots::{delete_bot_data, fold_bot}, clients::delete_client_data,
    conversations::{close_user_conversations, get_open, get_client_conversations},
    memories::{create_client_memory, get_memory, get_memories,
    delete_memories, delete_memory},
    messages::get_client_messages,
    state::get_client_current_state,
    clean_data::delete_expired_data,
    migrations::make_migrations,
    run, sns, validate
};

use csml_engine::{Client, data::RunRequest};
use csml_interpreter::data::csml_bot::CsmlBot;
use helpers::{format_response, format_csml_client};

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
    path_parameters: Option<serde_json::Value>,
    stage_variables: Option<serde_json::Value>,
    body: Option<String>,
    is_base64_encoded: bool,
}

fn lambda_handler(request: LambdaRequest, _c: Context) -> Result<serde_json::Value, HandlerError> {
    match request {

        /*
         * RUN
         */
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
            headers,
            ..
        } if path.ends_with("/sns") && http_method == "POST" => {

            Ok(sns::handler(headers, body))
        }


        /*
         * CONVERSATIONS
         */
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
            query_string_parameters: Some(query_params),
            ..
        } if path.ends_with("/conversations") && http_method == "GET" => {
            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            let limit = match query_params.get("limit") {
                Some(serde_json::Value::Number(limit)) => limit.as_i64(),
                _ => None
            };

            let pagination_key = match query_params.get("pagination_key") {
                Some(serde_json::Value::String(pagination_key)) => Some(pagination_key.to_owned()),
                _ => None
            };

            get_client_conversations(client, limit, pagination_key)
        }

        /*
         * MEMORIES
         */
        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            query_string_parameters: Some(query_params),
            ..
        } if path.ends_with("/memories") && http_method == "POST" => {
            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            let body: MemoryBody =  match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            create_client_memory(client, body.key, body.value)
        }

        LambdaRequest {
            path,
            http_method,
            query_string_parameters: Some(query_params),
            ..
        } if path.ends_with("/memories") && http_method == "DELETE" => {

            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            delete_memories(client)
        }

        LambdaRequest {
            path,
            http_method,
            query_string_parameters: Some(query_params),
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/memories/{key}") && http_method == "DELETE" => {

            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            let memory: MemoryKeyPath = match serde_json::from_value(path_params) {
                Ok(path_params) => {path_params},
                Err(_err) => return Ok(format_response(400, serde_json::json!("Path parameters bad format")))
            };

            delete_memory(client, &memory.key)
        }

        LambdaRequest {
            path,
            http_method,
            query_string_parameters: Some(query_params),
            ..
        } if path.ends_with("/memories") && http_method == "GET" => {

            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            get_memories(client)
        }

        LambdaRequest {
            path,
            http_method,
            query_string_parameters: Some(query_params),
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/memories/{key}") && http_method == "GET" => {

            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            let memory: MemoryKeyPath = match serde_json::from_value(path_params) {
                Ok(path_params) => {path_params},
                Err(_err) => return Ok(format_response(400, serde_json::json!("Path parameters bad format")))
            };

            get_memory(client, &memory.key)
        }

        /*
         * STATE
         */
        LambdaRequest {
            path,
            http_method,
            query_string_parameters: Some(query_params),
            ..
        } if path.ends_with("/state?") && http_method == "GET" => {

            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            get_client_current_state(client)
        }

        /*
         * MESSAGES
         */
        LambdaRequest {
            path,
            http_method,
            query_string_parameters: Some(query_params),
            ..
        } if path.ends_with("/messages") && http_method == "GET" => {

            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            let limit = match query_params.get("limit") {
                Some(serde_json::Value::Number(limit)) => limit.as_i64(),
                _ => None
            };

            let pagination_key = match query_params.get("pagination_key") {
                Some(serde_json::Value::String(pagination_key)) => Some(pagination_key.to_owned()),
                _ => None
            };

            get_client_messages(client, limit, pagination_key)
        }

        /*
         * CLIENTS
         */
        LambdaRequest {
            path,
            http_method,
            query_string_parameters: Some(query_params),
            ..
        } if path.ends_with("/data/clients") && http_method == "DELETE" => {

            let client = match format_csml_client(&query_params) {
                Ok(client) => client,
                Err(err) => return Ok(err),
            };

            delete_client_data(client)
        }

        LambdaRequest {
            path,
            http_method,
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/bots/{bot_id}") && http_method == "DELETE" => {

            let bot: BotIdPath = match serde_json::from_value(path_params) {
                Ok(path_params) => {path_params},
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            delete_bot_data(&bot.bot_id)
        }

        /*
         * VALIDATION
         */
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

        /*
         * BOTS
         */
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
            path_parameters: Some(path_params),
            ..
        } if path.ends_with("/bots/{bot_id}/versions") && http_method == "DELETE" => {
            let path_params: BotIdPath = match serde_json::from_value(path_params) {
                Ok(path_params) => {
                    path_params
                },
                Err(_err) => return Ok(format_response(400, serde_json::json!("Path parameters bad format")))
            };
            delete_bot_versions(path_params.bot_id)
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

            if let Some(serde_json::Value::Number(limit)) = query_params.get("limit") {
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
        } if path.ends_with("/bots/{bot_id}/versions/{version_id}") && http_method == "DELETE" => {
            let path_params: BotIdVersionIdPath = match serde_json::from_value(path_params) {
                Ok(path_params) => {
                    path_params
                },
                Err(_err) => return Ok(format_response(400, serde_json::json!("Path parameters bad format")))
            };

            delete_bot_version(path_params.bot_id, path_params.version_id)
        }


        LambdaRequest {
            path,
            http_method,
            body: Some(body),
            ..
        } if path.ends_with("/bots/fold") && http_method == "POST" => {
            let bot: CsmlBot = match serde_json::from_str(&body) {
                Ok(body) => body,
                Err(_err) => return Ok(format_response(400, serde_json::json!("Body bad format")))
            };

            fold_bot(bot)
        }


        /*
         * Delete all expired data in db (Conversation, Messages, Memory and State)
         */
        LambdaRequest {
            path,
            http_method,
            ..
        } if path.ends_with("/data/cleanup") && http_method == "POST" => {

            delete_expired_data()
        }


        /*
         * make migrations 
         */
        LambdaRequest {
            path,
            http_method,
            ..
        } if path.ends_with("/migrations") && http_method == "POST" => {

            make_migrations()
        }


        /*
         * CATCHALL
         */

        _ => Ok(format_response(404, serde_json::json!("Not found")))

    }
}
