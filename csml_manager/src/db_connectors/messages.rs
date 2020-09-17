use crate::{ConversationInfo, ManagerError};
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "http")]
use crate::db_connectors::{is_httpdb, http as http_connector};
#[cfg(feature = "dynamo")]
use crate::db_connectors::{is_dynamodb, dynamodb as dynamodb_connector};

pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    msgs: Vec<serde_json::Value>,
    interaction_order: i32,
    direction: &str,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::messages::add_messages_bulk(data, &msgs, interaction_order, direction);
    }

    #[cfg(feature = "http")]
    if is_httpdb() {
        return http_connector::messages::add_messages_bulk(data, &msgs, interaction_order, direction);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        return dynamodb_connector::messages::add_messages_bulk(data, &msgs, interaction_order, direction);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
