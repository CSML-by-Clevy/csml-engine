#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};

use csml_interpreter::data::csml_logs::{LogLvl, CsmlLog, csml_logger};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Database, ConversationInfo, EngineError, Client};
use crate::db_connectors::utils::*;

pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    msgs: Vec<serde_json::Value>,
    interaction_order: i32,
    direction: &str,
) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!("db call save messages {:?}", msgs)
        ),
        LogLvl::Info
    );
    csml_logger(
        CsmlLog::new(
            Some(&data.client),
            None,
            None,
            format!("db call save messages {:?}", msgs)
        ),
        LogLvl::Debug
    );

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let expires_at = get_expires_at_for_mongodb(data.ttl);

        return mongodb_connector::messages::add_messages_bulk(
            data,
            &msgs,
            interaction_order,
            direction,
            expires_at
        );
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let expires_at = get_expires_at_for_dynamodb(data.ttl);

        return dynamodb_connector::messages::add_messages_bulk(
            data,
            &msgs,
            interaction_order,
            direction,
            expires_at,
        );
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let expires_at = get_expires_at_for_postgresql(data.ttl);

        return postgresql_connector::messages::add_messages_bulk(
            data,
            &msgs,
            interaction_order,
            direction,
            expires_at,
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
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!("db call get messages")
        ),
        LogLvl::Info
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            format!("db call get messages")
        ),
        LogLvl::Debug
    );

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
