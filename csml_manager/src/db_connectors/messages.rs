use crate::{ConversationInfo, ManagerError};
use crate::db_connectors::{is_mongodb, is_http};
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::mongodb as mongodb_connector;
#[cfg(feature = "http")]
use crate::db_connectors::http as http_connector;

pub fn add_messages_bulk(
    data: &ConversationInfo,
    msgs: Vec<serde_json::Value>,
    interaction_order: i32,
    direction: &str,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::messages::add_messages_bulk(data, &msgs, interaction_order, direction);
    }

    #[cfg(feature = "http")]
    if is_http() {
        return http_connector::messages::add_messages_bulk(data, &msgs, interaction_order, direction);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
