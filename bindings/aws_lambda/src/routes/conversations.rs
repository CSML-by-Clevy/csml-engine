use csml_engine::{get_open_conversation, user_close_all_conversations, Client};

use crate::Error;

pub fn get_open(body: Client) -> Result<serde_json::Value, Error> {
    let res = get_open_conversation(&body);

    match res {
        Ok(Some(conversation)) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": serde_json::json!(conversation).to_string()
            }
        )),
        Ok(None) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": serde_json::json!(null).to_string()
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            eprintln!("{}", error);
            Err(Error::from(error.as_str()))
        }
    }
}

pub fn close_user_conversations(body: Client) -> Result<serde_json::Value, Error> {
    let res = user_close_all_conversations(body.clone());

    match res {
        Ok(()) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": serde_json::json!(null).to_string()
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            eprintln!("{}", error);
            Err(Error::from(error.as_str()))
        }
    }
}

pub fn get_client_conversations(
    client: Client,
    limit: Option<i64>,
    pagination_key: Option<String>,
) -> Result<serde_json::Value, Error> {
    let res = csml_engine::get_client_conversations(&client, limit, pagination_key);

    match res {
        Ok(conversations) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": conversations
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            eprintln!("{}", error);
            Err(Error::from(error.as_str()))
        }
    }
}
