#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};
#[cfg(feature = "sqlite")]
use crate::db_connectors::{is_sqlite, sqlite_connector};

use csml_interpreter::data::csml_logs::{csml_logger, CsmlLog, LogLvl};

use crate::db_connectors::utils::*;
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Client, ConversationInfo, Database, EngineError, Memory};
use std::collections::HashMap;

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &HashMap<String, Memory>,
) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!("db call save memories {:?}", memories.keys()),
        ),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!("db call save memories {:?}", memories.keys()),
        ),
        LogLvl::Debug,
    );

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
        return postgresql_connector::memories::add_memories(data, memories, expires_at);
    }

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let expires_at = get_expires_at_for_sqlite(data.ttl);
        return sqlite_connector::memories::add_memories(data, memories, expires_at);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn create_client_memory(
    client: &Client,
    key: String,
    value: serde_json::Value,
    ttl: Option<chrono::Duration>,
    db: &mut Database,
) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(None, None, None, format!("db call save memory {:?}", key)),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!("db call save memory {:?} with value {:?}", key, value),
        ),
        LogLvl::Debug,
    );

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        let expires_at = get_expires_at_for_mongodb(ttl);
        return mongodb_connector::memories::create_client_memory(
            client, key, value, expires_at, db,
        );
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        let expires_at = get_expires_at_for_dynamodb(ttl);

        return dynamodb_connector::memories::create_client_memory(
            client, key, value, expires_at, db,
        );
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        let expires_at = get_expires_at_for_postgresql(ttl);
        return postgresql_connector::memories::create_client_memory(
            client, &key, &value, expires_at, db,
        );
    }

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        let expires_at = get_expires_at_for_sqlite(ttl);
        return sqlite_connector::memories::create_client_memory(
            client, &key, &value, expires_at, db,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn internal_use_get_memories(
    client: &Client,
    db: &mut Database,
) -> Result<serde_json::Value, EngineError> {
    csml_logger(
        CsmlLog::new(None, None, None, "db call get memories".to_string()),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(Some(client), None, None, "db call get memories".to_string()),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::memories::internal_use_get_memories(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

/**
 * Get client Memories
 */
pub fn get_memories(client: &Client, db: &mut Database) -> Result<serde_json::Value, EngineError> {
    csml_logger(
        CsmlLog::new(None, None, None, "db call get memories client".to_string()),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            "db call get memories client".to_string(),
        ),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::memories::get_memories(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

/**
 * Get client Memory
 */
pub fn get_memory(
    client: &Client,
    key: &str,
    db: &mut Database,
) -> Result<serde_json::Value, EngineError> {
    csml_logger(
        CsmlLog::new(None, None, None, format!("db call get memory {:?}", key)),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            format!("db call get memory {:?}", key),
        ),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::memories::get_memory(client, key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn delete_client_memory(
    client: &Client,
    key: &str,
    db: &mut Database,
) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(None, None, None, format!("db call delete memory {:?}", key)),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            format!("db call delete memory {:?}", key),
        ),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::memories::delete_client_memory(client, key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn delete_client_memories(client: &Client, db: &mut Database) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(None, None, None, "db call delete memories".to_string()),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(Some(client), None, None, "db call delete memories".to_string()),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::memories::delete_client_memories(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
