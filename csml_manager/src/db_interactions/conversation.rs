use crate::{encrypt::encrypt_data, Client, ConversationInfo, ManagerError};
use bson::{doc, Bson};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Conversation {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: bson::oid::ObjectId,
    pub client: Client,
    pub flow_id: String,
    pub step_id: String,
    pub metadata: serde_json::Value, // encrypted
    pub status: String,              //(OPEN, CLOSED,  //Faild
    pub last_interaction_at: bson::UtcDateTime,
    pub updated_at: bson::UtcDateTime,
    pub created_at: bson::UtcDateTime,
}

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    metadata: serde_json::Value,
    db: &mongodb::Database,
) -> Result<bson::Bson, ManagerError> {
    let collection = db.collection("conversation");
    let time = Bson::UtcDatetime(chrono::Utc::now());
    let metadata = encrypt_data(&metadata)?;

    let conversation = doc! {
        "client": bson::to_bson(&client)?,
        "flow_id": flow_id,
        "step_id": step_id,
        "metadata": metadata, // encrypted
        "status": "OPEN",
        "last_interaction_at": &time,
        "updated_at": &time,
        "created_at": &time
    };

    let inserted = collection.insert_one(conversation.clone(), None)?;

    Ok(inserted.inserted_id)
}

pub fn close_conversation(
    id: &bson::Bson,
    client: &Client,
    db: &mongodb::Database,
) -> Result<(), ManagerError> {
    let collection = db.collection("conversation");

    let filter = doc! {
        "_id": id,
        "client": bson::to_bson(&client)?,
    };

    collection.update_one(
        filter,
        doc! {
            "$set": { "status": "CLOSE" },
            "$currentDate": { "last_interaction_at": true }
        },
        None,
    )?;
    Ok(())
}

pub fn close_all_conversations(
    client: &Client,
    db: &mongodb::Database,
) -> Result<(), ManagerError> {
    let collection = db.collection("conversation");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
    };

    collection.update_many(
        filter,
        doc! {
           "$set": { "status": "CLOSE" },
           "$currentDate": { "test": true }
        },
        None,
    )?;

    Ok(())
}

pub fn get_latest_open(
    client: &Client,
    db: &mongodb::Database,
) -> Result<Option<Conversation>, ManagerError> {
    let collection = db.collection("conversation");

    let filter = doc! {
        "status": "OPEN",
        "client": bson::to_bson(&client)?,
    };
    let find_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "$natural": -1 })
        .build();
    let result = collection.find_one(filter, find_options)?;

    match result {
        Some(conversation) => Ok(Some(bson::from_bson(bson::Bson::Document(conversation))?)),
        None => Ok(None),
    }
}

pub fn update_conversation(
    data: &ConversationInfo,
    flow_id: Option<String>,
    step_id: Option<String>,
) -> Result<(), ManagerError> {
    let collection = data.db.collection("conversation");

    let filter = doc! {
        "_id": &data.conversation_id,
        "client": bson::to_bson(&data.client)?,
    };

    let doc = match (flow_id, step_id) {
        (Some(flow_id), Some(step_id)) => doc! {
           "flow_id": flow_id,
           "step_id": step_id
        },
        (Some(flow_id), None) => doc! {
            "flow_id": flow_id
        },
        (None, Some(step_id)) => doc! {
           "step_id": step_id
        },
        (None, None) => doc! {},
    };

    let update = doc! {
        "$set": doc,
        "$currentDate": { "last_interaction_at": true }
    };

    collection.update_one(filter, update, None)?;
    Ok(())
}
