use serde_json::Value;

use crate::{
    Client, EngineError, MongoDbClient,
    encrypt::encrypt_data
};
use bson::{doc, Bson};

pub fn init_interaction(
    event: Value,
    client: &Client,
    db: &MongoDbClient,
) -> Result<String, EngineError> {
    let collection = db.client.collection("interaction");
    let time = Bson::DateTime(chrono::Utc::now());

    let doc = doc! {
        "client": bson::to_bson(&client)?,
        "event": encrypt_data(&event)?, // encrypted
        "updated_at": &time,
        "created_at": &time
    };

    let inserted = collection.insert_one(doc.clone(), None)?;

    let id = inserted.inserted_id.as_object_id().unwrap();

    Ok(id.to_hex())
}

pub fn update_interaction(
    interaction_id: &str,
    success: bool,
    client: &Client,
    db: &MongoDbClient,
) -> Result<(), EngineError> {
    let collection = db.client.collection("interaction");

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

pub fn delete_user_interactions(client: &Client, db: &MongoDbClient) -> Result<(), EngineError> {
    let collection = db.client.collection("interaction");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}