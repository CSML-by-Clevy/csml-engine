use crate::{Client, ContextJson, ConversationInfo, Database, ManagerError, Memories};

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &[Memories],
    interaction_order: i32,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::memories::add_memories as add;

        return add(data, &memories, interaction_order);
    }

    #[cfg(feature = "dynamo")]
    if cfg!(feature = "dynamo") {
        use crate::db_interactions::db_interactions_dynamo::memories::format_memories;
        use crate::db_interactions::db_interactions_dynamo::state::set_state_items;

        use crate::db_interactions::db_interactions_dynamo::get_db;

        println!("format memories");
        let mem = format_memories(data, memories, interaction_order);
        let db: &dynamodb::apis::client::APIClient = get_db(&data.db)?;

        println!("send memories");
        return set_state_items(&data.client, mem, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn get_memories(
    client: &Client,
    context: &mut ContextJson,
    metadata: &serde_json::Value,
    db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::get_db;
        use crate::db_interactions::db_interactions_mongo::memories::get_memories as get;

        let db: &mongodb::Database = get_db(db)?;

        return get(client, context, metadata, db);
    }

    #[cfg(feature = "dynamo")]
    if cfg!(feature = "dynamo") {
        use crate::db_interactions::db_interactions_dynamo::get_db;
        use crate::db_interactions::db_interactions_dynamo::state::get_state_type;

        let db: &dynamodb::apis::client::APIClient = get_db(db)?;

        let current = get_state_type(db, client, "remember")?;

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

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
