#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Client, ConversationInfo, Database, EngineError, Memory};
use std::collections::HashMap;

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &HashMap<String, Memory>,
) -> Result<(), EngineError> {

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::memories::add_memories(data, &memories);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        return dynamodb_connector::memories::add_memories(data, &memories);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        return postgresql_connector::memories::add_memories(data, &memories);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn create_client_memory(
    client: &Client,
    key: String,
    value: serde_json::Value,
    db: &mut Database
) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::memories::create_client_memory(client, key, value, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;

        return dynamodb_connector::memories::create_client_memory(client, key, value, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::memories::create_client_memory(client, &key, &value, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn internal_use_get_memories(client: &Client, db: &mut Database) -> Result<serde_json::Value, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::memories::internal_use_get_memories(client, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::memories::internal_use_get_memories(client, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::memories::internal_use_get_memories(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

/**
 * Get client Memories
 */
 pub fn get_memories(client: &Client, db: &mut Database) -> Result<serde_json::Value, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::memories::get_memories(client, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::memories::get_memories(client, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::memories::get_memories(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

/**
 * Get client Memory
 */
 pub fn get_memory(client: &Client, key: &str, db: &mut Database) -> Result<serde_json::Value, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::memories::get_memory(client, key, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::memories::get_memory(client, key, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::memories::get_memory(client, key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}


pub fn delete_client_memory(client: &Client, key: &str, db: &mut Database) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::memories::delete_client_memory(client, key, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::memories::delete_client_memory(client, key, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::memories::delete_client_memory(client, key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn delete_client_memories(client: &Client, db: &mut Database) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::memories::delete_client_memories(client, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::memories::delete_client_memories(client, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::memories::delete_client_memories(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
