use crate::{Client, ConversationInfo, ManagerError, Database};
use bson::{doc, Bson};

pub fn create_node(data: &mut ConversationInfo) -> Result<(), ManagerError> {
    let time = Bson::UtcDatetime(chrono::Utc::now());
    

    let db: &mongodb::Database = if let Database::Mongo(ref db) = data.db {
        db
    } else {
        return Err (
            ManagerError::Manager("db is not init correctly".to_owned())
        )
    };
    
    let path = db.collection("path");

    let node = doc! {
        "client": bson::to_bson(&data.client)?,
        "interaction_id": &data.interaction_id,
        "conversation_id": &data.conversation_id,
        "flow_id": &data.context.flow,
        "step_id": &data.context.step,
        "next_flow": Bson::Null, //"Option<string>",
        "next_step": Bson::Null, //"Option<string>",
        "created_at": time
    };

    path.insert_one(node, None)?;

    Ok(())
}
