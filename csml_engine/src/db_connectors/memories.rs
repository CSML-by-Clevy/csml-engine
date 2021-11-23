#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Client, ConversationInfo, Database, EngineError, Memory};
use crate::db_connectors::utils::*;
use std::collections::HashMap;

use log::{debug, info,};

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &HashMap<String, Memory>,
) -> Result<(), EngineError> {

    info!("db call save memories {:?}", memories.keys());
    debug!("db call save memories {:?}", memories);

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let expires_at = get_expires_at_for_mongodb(data.ttl);
        return mongodb_connector::memories::add_memories(data, &memories, expires_at);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let expires_at = get_expires_at_for_dynamodb(data.ttl);
        return dynamodb_connector::memories::add_memories(data, &memories, expires_at);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let expires_at = get_expires_at_for_postgresql(data.ttl);
        return postgresql_connector::memories::add_memories(data, &memories, expires_at);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn create_client_memory(
    client: &Client,
    key: String,
    value: serde_json::Value,
    ttl: Option<chrono::Duration>,
    db: &mut Database
) -> Result<(), EngineError> {

    info!("db call save memory {:?}", key);
    debug!("db call save memory {:?} with value {:?}", key, value);

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        let expires_at = get_expires_at_for_mongodb(ttl);
        return mongodb_connector::memories::create_client_memory(client, key, value, expires_at, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        let expires_at = get_expires_at_for_dynamodb(ttl);

        return dynamodb_connector::memories::create_client_memory(client, key, value, expires_at, db);
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        let expires_at = get_expires_at_for_postgresql(ttl);
        return postgresql_connector::memories::create_client_memory(client, &key, &value, expires_at,db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn internal_use_get_memories(client: &Client, db: &mut Database) -> Result<serde_json::Value, EngineError> {
    info!("db call get memories");
    debug!("db call get memories client {:?}", client);

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
    info!("db call get memories client");
    debug!("db call get memories client {:?}", client);

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
    info!("db call get memory {:?}", key);
    debug!("db call get memory {:?}, client: {:?}", key, client);

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
    info!("db call delete memory {:?}", key);
    debug!("db call delete memory {:?}, client: {:?}", key, client);

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
    info!("db call delete memories");
    debug!("db call delete memories, client: {:?}", client);

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
