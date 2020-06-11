use crate::{Client, ConversationInfo, ManagerError, Database};

pub fn init_interaction(
    event: serde_json::Value,
    client: &Client,
    db: &Database,
) -> Result<String, ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::interactions::init_interaction as init;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(db)?;

        return init(event, client, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}

pub fn update_interaction(
    data: &ConversationInfo,
    success: bool
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::interactions::update_interaction as update;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(&data.db)?;

        return update(&data.interaction_id, success, &data.client, db)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}
