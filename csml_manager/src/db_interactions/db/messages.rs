use crate::{Client, ConversationInfo, ManagerError, Message};

pub fn add_messages_bulk(
    data: &ConversationInfo,
    msgs: Vec<serde_json::Value>,
    interaction_order: i32,
    direction: &str,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    {
        use crate::db_interactions::db_interactions_mongo::messages::add_messages_bulk as add;

        return add(data, &msgs, interaction_order, direction)
    }

    Err (
        ManagerError::Manager("db is not init correctly".to_owned())
    )
}
