#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb_connector};
#[cfg(feature = "postgresql")]
use crate::db_connectors::{is_postgresql, postgresql_connector};
#[cfg(feature = "sqlite")]
use crate::db_connectors::{is_sqlite, sqlite_connector};

use csml_interpreter::data::csml_logs::{csml_logger, CsmlLog, LogLvl};

use crate::db_connectors::{state, utils::*};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Client, ConversationInfo, Database, DbConversation, EngineError};

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    ttl: Option<chrono::Duration>,
    db: &mut Database,
) -> Result<String, EngineError> {
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!(
                "db call create conversation flow_id: {}, step_id:{}",
                flow_id, step_id
            ),
        ),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            format!(
                "db call create conversation flow_id: {}, step_id:{}",
                flow_id, step_id
            ),
        ),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        let expires_at = get_expires_at_for_sqlite(ttl);
        return sqlite_connector::conversations::create_conversation(
            flow_id, step_id, client, expires_at, db,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn close_conversation(id: &str, client: &Client, db: &mut Database) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!("db call close conversation conversation_id: {}", id),
        ),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            format!("db call close conversation conversation_id: {}", id),
        ),
        LogLvl::Debug,
    );

    // delete previous bot info at the end of the conversation
    state::delete_state_key(client, "bot", "previous", db)?;

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::conversations::close_conversation(id, client, "CLOSED", db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn close_all_conversations(client: &Client, db: &mut Database) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(None, None, None, "db call close all conversations".to_string()),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            format!("db call close all conversations, client: {:?}", client),
        ),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::conversations::close_all_conversations(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_latest_open(
    client: &Client,
    db: &mut Database,
) -> Result<Option<DbConversation>, EngineError> {
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            "db call get latest open conversations".to_string(),
        ),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            "db call get latest open conversations".to_string(),
        ),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::conversations::get_latest_open(client, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn update_conversation(
    data: &mut ConversationInfo,
    flow_id: Option<String>,
    step_id: Option<String>,
) -> Result<(), EngineError> {
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!(
                "db call update conversations flow_id {:?}, step_id {:?}",
                flow_id, step_id
            ),
        ),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(&data.client),
            None,
            None,
            format!(
                "db call update conversations flow_id {:?}, step_id {:?}",
                flow_id, step_id
            ),
        ),
        LogLvl::Debug,
    );

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

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(&mut data.db)?;
        return sqlite_connector::conversations::update_conversation(
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
    csml_logger(
        CsmlLog::new(
            None,
            None,
            None,
            format!("db call get client conversations, limit: {:?}", limit),
        ),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(client),
            None,
            None,
            format!(
                "db call get client conversations limit: {:?}, pagination_key: {:?}",
                limit, pagination_key
            ),
        ),
        LogLvl::Info,
    );

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        let pagination_key = mongodb_connector::get_pagination_key(pagination_key)?;

        return mongodb_connector::conversations::get_client_conversations(
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

        return dynamodb_connector::conversations::get_client_conversations(
            client,
            db,
            limit,
            pagination_key,
        );
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        let db = postgresql_connector::get_db(db)?;
        return postgresql_connector::conversations::get_client_conversations(
            client,
            db,
            limit,
            pagination_key,
        );
    }

    #[cfg(feature = "sqlite")]
    if is_sqlite() {
        let db = sqlite_connector::get_db(db)?;
        return sqlite_connector::conversations::get_client_conversations(
            client,
            db,
            limit,
            pagination_key,
        );
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
