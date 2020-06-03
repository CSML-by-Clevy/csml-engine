use serde_json::Value;

use crate::{Client, ConversationInfo, ManagerError};
use bson::{doc, Bson};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Interaction {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: bson::oid::ObjectId,
    pub client: Client,
    pub success: bool,
    pub event: bson::Document, // encrypted
    pub updated_at: bson::UtcDateTime,
    pub created_at: bson::UtcDateTime,
}

pub fn init_interaction(
    event: Value,
    client: &Client,
    db: &mongodb::Database,
) -> Result<bson::Bson, ManagerError> {
    let collection = db.collection("interaction");
    let time = Bson::UtcDatetime(chrono::Utc::now());

    let doc = doc! {
        "client": bson::to_bson(&client)?,
        // "success": "boolean",
        "event": event, // encrypted
        "updated_at": &time,
        "created_at": &time
    };

    let insserted = collection.insert_one(doc.clone(), None)?;

    Ok(insserted.inserted_id)
}

pub fn update_interaction(data: &ConversationInfo, success: bool) -> Result<(), ManagerError> {
    let collection = data.db.collection("interaction");

    let filter = doc! {
        "_id": &data.interaction_id,
        "client": bson::to_bson(&data.client)?,
    };

    collection.update_one(
        filter,
        doc! {
           "$set": { "success": success },
           "$currentDate": { "updated_at": true }
        },
        None,
    )?;

    Ok(())
}
