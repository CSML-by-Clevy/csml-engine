use serde_json::Value;

use crate::{Client, ConversationInfo, ManagerError, db_interactions::Interaction};
use bson::{doc, Bson};
use chrono::{prelude::Utc, SecondsFormat};


pub fn init_interaction(
    event: Value,
    client: &Client,
    db: &mongodb::Database,
) -> Result<String, ManagerError> {
    let collection = db.collection("interaction");
    let time = Bson::UtcDatetime(chrono::Utc::now());

    let doc = doc! {
        "client": bson::to_bson(&client)?,
        // "success": "boolean",
        "event": event, // encrypted
        "updated_at": &time,
        "created_at": &time
    };

    let inserted = collection.insert_one(doc.clone(), None)?;

    let id = inserted.inserted_id.as_object_id().unwrap();

    Ok(id.to_hex())
}

pub fn update_interaction(interaction_id: &str, success: bool, client: &Client, db: &mongodb::Database) -> Result<(), ManagerError> {
    let collection = db.collection("interaction");

    let filter = doc! {
        "_id": bson::oid::ObjectId::with_string(interaction_id).unwrap(),
        "client": bson::to_bson(&client)?,
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
