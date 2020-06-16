use crate::{ConversationInfo, ManagerError};

pub fn add_messages_bulk(
    data: &ConversationInfo,
    msgs: Vec<serde_json::Value>,
    interaction_order: i32,
    direction: &str,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::messages::add_messages_bulk as add;

        return add(data, &msgs, interaction_order, direction);
    }

    #[cfg(feature = "dynamo")]
    if cfg!(feature = "dynamo") {
        use crate::db_interactions::db_interactions_dynamo::messages::add_messages_bulk as add;
        // use crate::db_interactions::db_interactions_dynamo::get_db;
        // let db: &dynamodb::apis::client::APIClient = get_db(db)?;

        return add(data, &msgs, interaction_order, direction);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
