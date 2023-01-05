use csml_engine::Client;

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

pub fn format_csml_client(query_params: &serde_json::Value) -> Result<Client, serde_json::Value> {
    match (
        query_params.get("user_id"),
        query_params.get("bot_id"),
        query_params.get("channel_id"),
    ) {
        (
            Some(serde_json::Value::String(user_id)),
            Some(serde_json::Value::String(bot_id)),
            Some(serde_json::Value::String(channel_id)),
        ) => Ok(Client {
            user_id: user_id.to_owned(),
            bot_id: bot_id.to_owned(),
            channel_id: channel_id.to_owned(),
        }),
        _ => {
            return Err(format_response(
                400,
                serde_json::json!("Missing query params client info (user_id, bot_id, channel_id)"),
            ))
        }
    }
}
