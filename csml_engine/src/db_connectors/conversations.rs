#[cfg(feature = "dynamo")]
use crate::db_connectors::{is_dynamodb, dynamodb_connector};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};

use crate::error_messages::ERROR_DB_SETUP;
use crate::{Client, ConversationInfo, Database, DbConversation, EngineError};
use crate::db_connectors::utils::*;

use log::{debug, info,};

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    ttl: Option<chrono::Duration>,
    db: &mut Database,
) -> Result<String, EngineError> {
    info!("db call create conversation flow_id: {}, step_id:{}", flow_id, step_id);
    debug!("db call create conversation flow_id: {}, step_id:{}, client: {:?}", flow_id, step_id, client);

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;

        let expires_at = get_expires_at_for_mongodb(ttl);
        return mongodb_connector::conversations::create_conversation(
            flow_id, step_id, client, expires_at, db,
        );
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        let expires_at = get_expires_at_for_dynamodb(ttl);
        return dynamodb_connector::conversations::create_conversation(
            flow_id, step_id, client, expires_at, db,
        );
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        let expires_at = get_expires_at_for_postgresql(ttl);
        return postgresql_connector::conversations::create_conversation(
            flow_id, step_id, client, expires_at, db,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn close_conversation(id: &str, client: &Client, db: &mut Database) -> Result<(), EngineError> {
    info!("db call close conversation conversation_id: {}", id);
    debug!("db call close conversation conversation_id: {}, client: {:?}", id, client);

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::conversations::close_conversation(id, client, "CLOSED", db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::conversations::close_conversation(id, client, "CLOSED", db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::conversations::close_conversation(id, client, "CLOSED", db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn close_all_conversations(client: &Client, db: &mut Database) -> Result<(), EngineError> {
    info!("db call close all conversations");
    debug!("db call close all conversations, client: {:?}", client);

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::conversations::close_all_conversations(client, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::conversations::close_all_conversations(client, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::conversations::close_all_conversations(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_latest_open(
    client: &Client,
    db: &mut Database,
) -> Result<Option<DbConversation>, EngineError> {
    info!("db call get latest open conversations");
    debug!("db call get latest open conversations, client: {:?}", client);

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::conversations::get_latest_open(client, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::conversations::get_latest_open(client, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::conversations::get_latest_open(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn update_conversation(
    data: &mut ConversationInfo,
    flow_id: Option<String>,
    step_id: Option<String>,
) -> Result<(), EngineError> {
    info!("db call update conversations flow_id {:?}, step_id {:?}", flow_id, step_id);
    debug!("db call update conversations flow_id {:?}, step_id {:?}, client {:?}", flow_id, step_id, data.client);

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(&data.db)?;
        return mongodb_connector::conversations::update_conversation(
            &data.conversation_id,
            &data.client,
            flow_id,
            step_id,
            db,
        );
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(&mut data.db)?;
        return dynamodb_connector::conversations::update_conversation(
            &data.conversation_id,
            &data.client,
            flow_id,
            step_id,
            db,
        );
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(&mut data.db)?;
        return postgresql_connector::conversations::update_conversation(
            &data.conversation_id,
            flow_id,
            step_id,
            db,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_client_conversations(
    client: &Client,
    db: &mut Database,
    limit: Option<i64>,
    pagination_key: Option<String>,
) -> Result<serde_json::Value, EngineError> {
    info!("db call get client conversations, limit: {:?}", limit);
    debug!("db call get client conversations limit: {:?}, pagination_key: {:?}, client {:?}", limit, pagination_key, client);

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        let pagination_key = mongodb_connector::get_pagination_key(pagination_key)?;

        return mongodb_connector::conversations::get_client_conversations(
            client,
            db,
            limit,
            pagination_key
        );
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        let pagination_key = dynamodb_connector::get_pagination_key(pagination_key)?;

        return dynamodb_connector::conversations::get_client_conversations(
            client,
            db,
            limit,
            pagination_key
        );
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::conversations::get_client_conversations(
            client,
            db,
            limit,
            pagination_key
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
