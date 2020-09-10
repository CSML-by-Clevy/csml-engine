use crate::{Client, DbConversation, ConversationInfo, Database, ManagerError};
use crate::db_connectors::{is_mongodb, is_http};
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "http")]
use crate::db_connectors::{is_http, http as http_connector};

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    metadata: serde_json::Value,
    db: &Database,
) -> Result<String, ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::conversations::create_conversation(flow_id, step_id, client, metadata, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db = http_connector::get_db(db)?;
        return http_connector::conversations::create_conversation(flow_id, step_id, client, metadata, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn close_conversation(id: &str, client: &Client, db: &Database) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::conversations::close_conversation(id, client, "CLOSED", db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db = http_connector::get_db(db)?;
        return http_connector::conversations::close_conversation(id, client, "CLOSED", db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn close_all_conversations(client: &Client, db: &Database) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::conversations::close_all_conversations(client, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db = http_connector::get_db(db)?;
        return http_connector::conversations::close_all_conversations(client, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_latest_open(
    client: &Client,
    db: &Database,
) -> Result<Option<DbConversation>, ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::conversations::get_latest_open(client, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db = http_connector::get_db(db)?;
        return http_connector::conversations::get_latest_open(client, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn update_conversation(
    data: &ConversationInfo,
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

    #[cfg(feature = "http")]
    if is_http() {
        let db = http_connector::get_db(&data.db)?;
        return http_connector::conversations::update_conversation(
            &data.conversation_id,
            &data.client,
            flow_id,
            step_id,
            db,
        );
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
