use serde_json::Value;

use crate::{Client, ConversationInfo, ManagerError, Database};

pub fn init_interaction(
    event: Value,
    client: &Client,
    db: &Database,
) -> Result<String, ManagerError> {

    unimplemented!()
    // Ok(insserted.inserted_id)
}

pub fn update_interaction(data: &ConversationInfo, success: bool) -> Result<(), ManagerError> {
    unimplemented!()

    // Ok(())
}
