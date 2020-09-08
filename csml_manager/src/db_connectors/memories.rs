use crate::{Client, ContextJson, ConversationInfo, Database, ManagerError, Memories};
use crate::db_connectors::{is_mongodb, is_http};
use crate::error_messages::ERROR_DB_SETUP;
#[cfg(feature = "mongo")]
use crate::db_connectors::mongodb as mongodb_connector;
#[cfg(feature = "http")]
use crate::db_connectors::http as http_connector;

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
        let db: &http_db::apis::client::APIClient = http_connector::get_db(&data.db)?;

        return set_state_items(&data.client, mem, db);
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_memories(
    client: &Client,
    context: &mut ContextJson,
    metadata: &serde_json::Value,
    db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db: &mongodb::Database = mongodb_connector::get_db(db)?;
        return mongodb_connector::memories::get_memories(client, context, metadata, db);
    }

    #[cfg(feature = "http")]
    if is_http() {
        let db: &http_db::apis::client::APIClient = http_connector::get_db(db)?;

        let current = http_connector::state::get_state_type(db, client, "remember")?;

        let map = current.iter().fold(serde_json::Map::new(), |mut map, mem| {
            if !map.contains_key(&mem.key) {
                map.insert(mem.key.clone(), mem.value.clone());
            }
            map
        });
        context.current = serde_json::json!(map);
        context.metadata = metadata.clone();

        return Ok(());
    }

    Err(ManagerError::Manager(ERROR_DB_SETUP.to_owned()))
}
