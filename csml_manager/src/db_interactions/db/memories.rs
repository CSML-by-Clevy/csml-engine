use crate::{
    Database, Client, ContextJson, ConversationInfo, ManagerError, Memories,
};

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &[Memories]
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::memories::add_memories as add;

        return add(data, &memories)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}

pub fn get_memories(
    client: &Client,
    context: &mut ContextJson,
    metadata: &serde_json::Value,
    db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::memories::get_memories as get;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;

        return get(client, context, metadata, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}
