use crate::{Client, Conversation, ConversationInfo, ManagerError, Database};

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    metadata: serde_json::Value,
    db: &Database,
) -> Result<String, ManagerError> {

    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::conversation::create_conversation as create;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;

        return create(flow_id, step_id, client, metadata, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}

pub fn close_conversation(
    id: &String,
    client: &Client,
    db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::conversation::close_conversation as close;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;

        return close(id, client, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}

pub fn close_all_conversations(
    client: &Client,
    db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::conversation::close_all_conversations as close_all;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;

        return close_all(client, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}

pub fn get_latest_open(
    client: &Client,
    db: &Database,
) -> Result<Option<Conversation>, ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::conversation::get_latest_open as get_latest;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;
        return  get_latest(client, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}

pub fn update_conversation(
    data: &ConversationInfo,
    flow_id: Option<String>,
    step_id: Option<String>,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::conversation::update_conversation as update;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(&data.db)?;

        return update(data.conversation_id.clone(), &data.client, flow_id, step_id, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}
