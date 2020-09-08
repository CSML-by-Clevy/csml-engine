use crate::{ConversationInfo, Database, ManagerError};
use csml_interpreter::data::Client;
use crate::error_messages::ERROR_DB_SETUP;
use crate::db_connectors::{is_mongodb, is_http};
#[cfg(feature = "mongo")]
use crate::db_connectors::mongodb as mongodb_connector;
#[cfg(feature = "http")]
use crate::db_connectors::http as http_connector;

pub fn delete_state_key(
    _client: &Client,
    _type: &str,
    _key: &str,
    _db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db: &mongodb::Database = mongodb_connector::get_db(_db)?;
        return mongodb_connector::state::delete_state_key(_client, _type, _key, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db: &http_db::apis::client::APIClient = http_connector::get_db(_db)?;
        return http_connector::state::delete_state_key(_client, _type, _key, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_state_key(
    _client: &Client,
    _type: &str,
    _key: &str,
    _db: &Database,
) -> Result<Option<serde_json::Value>, ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db: &mongodb::Database = mongodb_connector::get_db(_db)?;
        return mongodb_connector::state::get_state_key(_client, _type, _key, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db: &http_db::apis::client::APIClient = http_connector::get_db(_db)?;
        return http_connector::state::get_state_key(_client, _type, _key, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn set_state_items(
    _data: &mut ConversationInfo,
    _type: &str,
    _interaction_order: i32,
    _keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), ManagerError> {
    // Document
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::state::set_state_items(_data, _type, _keys_values);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let state_body = http_connector::state::format_state_body(_data, _type, _interaction_order, _keys_values);
        let db: &http_db::apis::client::APIClient = http_connector::get_db(&_data.db)?;
        return http_connector::state::set_state_items(&_data.client, state_body, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
