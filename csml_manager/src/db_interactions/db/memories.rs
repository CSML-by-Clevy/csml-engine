use crate::{
    Database, Client, ContextJson, ConversationInfo, ManagerError, Memories,
};

// pub fn format_memories(
//     data: &mut ConversationInfo,
//     memories: &[Memories],
// ) -> Result<Vec<Document>, ManagerError> {
//     unimplemented!()
//     // vec
// }

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: Vec<i32>, // Document or json value ?
) -> Result<(), ManagerError> {
    unimplemented!()
    // Ok(())
}

pub fn get_memories(
    client: &Client,
    // conversation_id: &bson::Bson,
    context: &mut ContextJson,
    metadata: &serde_json::Value,
    db: &Database,
) -> Result<(), ManagerError> {
    unimplemented!()
    // Ok(())
}
