#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Database, ConversationInfo, EngineError, Client};

pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    msgs: Vec<serde_json::Value>,
    interaction_order: i32,
    direction: &str,
) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::messages::add_messages_bulk(
            data,
            &msgs,
            interaction_order,
            direction,
        );
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        return dynamodb_connector::messages::add_messages_bulk(
            data,
            &msgs,
            interaction_order,
            direction,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_conversation_messages(
    client: &Client,
    conversation_id: &str,
    db: &mut Database,
    limit: Option<i64>,
    pagination_key: Option<String>,
) -> Result<serde_json::Value, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;

        return mongodb_connector::messages::get_conversation_messages(
            client,
            conversation_id,
            db,
            limit,
            pagination_key,
        );
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;

        let pagination_key = match pagination_key {
            Some(key) => {
                let base64decoded = match base64::decode(&key) {
                    Ok(base64decoded) => base64decoded,
                    Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
                };
                match serde_json::from_slice(&base64decoded) {
                    Ok(key) => Some(key),
                    Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
                }
            }
            None => None,
        };

        return dynamodb_connector::messages::get_conversation_messages(
            client,
            conversation_id,
            db,
            limit,
            pagination_key,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
