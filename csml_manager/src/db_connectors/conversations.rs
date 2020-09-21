use crate::{Client, DbConversation, ConversationInfo, Database, ManagerError};
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "dynamo")]
use crate::db_connectors::{is_dynamodb, dynamodb as dynamodb_connector};

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    metadata: serde_json::Value,
    db: &mut Database,
) -> Result<String, ManagerError> {

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::conversations::create_conversation(flow_id, step_id, client, metadata, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::conversations::create_conversation(flow_id, step_id, client, metadata, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn close_conversation(id: &str, client: &Client, db: &mut Database) -> Result<(), ManagerError> {
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

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn close_all_conversations(client: &Client, db: &mut Database) -> Result<(), ManagerError> {
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

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_latest_open(
    client: &Client,
    db: &mut Database,
) -> Result<Option<DbConversation>, ManagerError> {
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

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn update_conversation(
    data: &mut ConversationInfo,
    flow_id: Option<String>,
    step_id: Option<String>,
) -> Result<(), ManagerError> {
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

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
