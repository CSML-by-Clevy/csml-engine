#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{ConversationInfo, EngineError};

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
