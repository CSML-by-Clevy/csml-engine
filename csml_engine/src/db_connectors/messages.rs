#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};
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

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        return postgresql_connector::messages::add_messages_bulk(
            data,
            &msgs,
            interaction_order,
            direction,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_client_messages(
    client: &Client,
    db: &mut Database,
    limit: Option<i64>,
    pagination_key: Option<String>,
) -> Result<serde_json::Value, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        let pagination_key = mongodb_connector::get_pagination_key(pagination_key)?;

        return mongodb_connector::messages::get_client_messages(
            client,
            db,
            limit,
            pagination_key,
        );
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        let pagination_key = dynamodb_connector::get_pagination_key(pagination_key)?;

        return dynamodb_connector::messages::get_client_messages(
            client,
            db,
            limit,
            pagination_key,
        );
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;

        return postgresql_connector::messages::get_client_messages(
            client,
            db,
            limit,
            pagination_key,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
