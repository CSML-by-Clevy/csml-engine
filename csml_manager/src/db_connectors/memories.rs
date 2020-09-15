use crate::{Client, ConversationInfo, Database, ManagerError, Memories};
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
#[cfg(feature = "http")]
use crate::db_connectors::{is_http, http as http_connector};

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &[Memories],
    interaction_order: i32,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::memories::add_memories(data, &memories, interaction_order);
    }

    #[cfg(feature = "http")]
    if is_http() {
        use http_connector::{memories::format_memories, state::set_state_items};

        let mem = format_memories(data, memories, interaction_order);
        let db = http_connector::get_db(&data.db)?;

        return set_state_items(&data.client, mem, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

/**
 * Memories will be injected into the conversation's current context
 * so `context` must be mutable.
 */
pub fn get_memories(
    client: &Client,
    db: &Database,
) -> Result<serde_json::Value, ManagerError> {

    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::memories::get_memories(client, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db = http_connector::get_db(db)?;
        return http_connector::memories::get_memories(client, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
