use crate::{ConversationInfo, Database, ManagerError};
use csml_interpreter::data::Client;
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "http")]
use crate::db_connectors::{is_http, http as http_connector};

pub fn delete_state_key(
    client: &Client,
    _type: &str,
    _key: &str,
    db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::state::delete_state_key(client, _type, _key, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db = http_connector::get_db(db)?;
        return http_connector::state::delete_state_key(client, _type, _key, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_state_key(
    client: &Client,
    _type: &str,
    _key: &str,
    db: &Database,
) -> Result<Option<serde_json::Value>, ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::state::get_state_key(client, _type, _key, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db = http_connector::get_db(db)?;
        return http_connector::state::get_state_key(client, _type, _key, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn set_state_items(
    data: &mut ConversationInfo,
    _type: &str,
    interaction_order: i32,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), ManagerError> {

    #[cfg(feature = "mongo")]
    if is_mongodb() {
    let state_data = mongodb_connector::state::format_state_data(&data.client, _type, keys_values)?;
    let db = mongodb_connector::get_db(&data.db)?;
        return mongodb_connector::state::set_state_items(&data.client, state_data, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let state_data = http_connector::state::format_state_data(data, _type, interaction_order, keys_values);
        let db = http_connector::get_db(&data.db)?;
        return http_connector::state::set_state_items(&data.client, state_data, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
