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

        let db: &mongodb::Database = if let Database::Mongo(mongo) = db {
            mongo
        } else {
            return Err (
                ManagerError::Manager("db is not init correctly".to_owned())
            )
        };

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

        let db: &mongodb::Database = if let Database::Mongo(db) = db {
            db
        } else {
            return Err (
                ManagerError::Manager("db is not init correctly".to_owned())
            )
        };

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

        let db: &mongodb::Database = if let Database::Mongo(db) = db {
            db
        } else {
            return Err (
                ManagerError::Manager("db is not init correctly".to_owned())
            )
        };

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

        let db: &mongodb::Database = if let Database::Mongo(db) = db {
            db
        } else {
            return Err (
                ManagerError::Manager("db is not init correctly".to_owned())
            )
        };

        return get_latest(client, db)
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

        let db: &mongodb::Database = if let Database::Mongo(ref db) = data.db {
            db
        } else {
            return Err (
                ManagerError::Manager("db is not init correctly".to_owned())
            )
        };

        return update(data.conversation_id.clone(), &data.client, flow_id, step_id, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}
