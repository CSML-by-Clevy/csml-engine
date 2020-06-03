use crate::{Client, ConversationInfo, ManagerError};
use bson::{doc, Bson};
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Node {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: bson::oid::ObjectId,
    pub client: Client,
    pub interaction_id: bson::oid::ObjectId,
    pub conversation_id: bson::oid::ObjectId,
    pub flow_id: String,
    pub step_id: String,
    pub next_step: Option<String>,
    pub next_flow: Option<String>,
    pub created_at: bson::UtcDateTime,
}

pub fn create_node(data: &mut ConversationInfo) -> Result<(), ManagerError> {
    let time = Bson::UtcDatetime(chrono::Utc::now());
    let path = data.db.collection("path");

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
