use crate::{db_connectors::DbConversation, encrypt::{encrypt_data, decrypt_data}, Client, EngineError};
use bson::{doc, Bson};
use chrono::SecondsFormat;

fn format_conversation_struct(
    conversation: bson::ordered::OrderedDocument,
) -> Result<DbConversation, EngineError> {
    let encrypted_metadata: String = conversation.get_str("metadata").unwrap().to_owned();
    let metadata = decrypt_data(encrypted_metadata)?;

    Ok(DbConversation {
        id: conversation.get_object_id("_id").unwrap().to_hex(), // to_hex bson::oid::ObjectId
        client: bson::from_bson(conversation.get("client").unwrap().to_owned())?,
        flow_id: conversation.get_str("flow_id").unwrap().to_owned(), // to_hex
        step_id: conversation.get_str("step_id").unwrap().to_owned(), // to_hex
        metadata, // encrypted
        status: conversation.get_str("status").unwrap().to_owned(),   //(OPEN, CLOSED, //Faild?
        last_interaction_at: conversation
            .get_utc_datetime("last_interaction_at")
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
        updated_at: conversation
            .get_utc_datetime("updated_at")
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
        created_at: conversation
            .get_utc_datetime("created_at")
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
    })
}

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    metadata: serde_json::Value,
    db: &mongodb::Database,
) -> Result<String, EngineError> {
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

    let id = inserted.inserted_id.as_object_id().unwrap();

    Ok(id.to_hex())
}

pub fn close_conversation(
    id: &str,
    client: &Client,
    status: &str,
    db: &mongodb::Database,
) -> Result<(), EngineError> {
    let collection = db.collection("conversation");

    let filter = doc! {
        "_id": bson::oid::ObjectId::with_string(id).unwrap(),
        "client": bson::to_bson(&client)?,
    };

    collection.update_one(
        filter,
        doc! {
            "$set": { "status": status },
            "$currentDate": { "last_interaction_at": true }
        },
        None,
    )?;
    Ok(())
}

pub fn close_all_conversations(client: &Client, db: &mongodb::Database) -> Result<(), EngineError> {
    let collection = db.collection("conversation");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
    };

    collection.update_many(
        filter,
        doc! {
           "$set": { "status": "CLOSED" },
           "$currentDate": { "test": true }
        },
        None,
    )?;

    Ok(())
}

pub fn get_latest_open(
    client: &Client,
    db: &mongodb::Database,
) -> Result<Option<DbConversation>, EngineError> {
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
        Some(conv) => {
            let conversation = format_conversation_struct(conv)?;
            Ok(Some(conversation))
        }
        None => Ok(None),
    }
}

pub fn update_conversation(
    conversation_id: &str,
    client: &Client,
    flow_id: Option<String>,
    step_id: Option<String>,
    db: &mongodb::Database,
) -> Result<(), EngineError> {
    let collection = db.collection("conversation");

    let filter = doc! {
        "_id": bson::oid::ObjectId::with_string(conversation_id).unwrap(),
        "client": bson::to_bson(&client)?,
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

pub fn delete_user_conversations(client: &Client, db: &mongodb::Database) -> Result<(), EngineError> {
    let collection = db.collection("conversation");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}

pub fn get_client_conversations(
    client: &Client,
    db: &mongodb::Database,
    limit: Option<i64>,
    pagination_key: Option<String>,
) -> Result<serde_json::Value, EngineError> {
    let collection = db.collection("conversation");

    let limit = match limit {
        Some(limit) if limit >= 1 => limit + 1,
        Some(_limit) => 21,
        None => 21,
    };

    let filter = match pagination_key {
        Some(key) => {
            let base64decoded = match base64::decode(&key) {
                Ok(base64decoded) => base64decoded,
                Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
            };
    
            let key: String = match serde_json::from_slice(&base64decoded) {
                Ok(key) => key,
                Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
            };

            doc! {
                "client": bson::to_bson(&client)?,
                "_id": {"$gt": bson::oid::ObjectId::with_string(&key).unwrap() }
            }
        }
        None => doc! {"client": bson::to_bson(&client)?},
    };

    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "$natural": -1 })
        .batch_size(30)
        .limit(limit)
        .build();
    let cursor = collection.find(filter, find_options)?;

    let mut conversations = vec![];
    for doc in cursor {
        match doc {
            Ok(conv) => {
                let conversation = format_conversation_struct(conv)?;

                let json = serde_json::json!({
                    "client": conversation.client,
                    "flow_id": conversation.flow_id,
                    "step_id": conversation.step_id,
                    "metadata": conversation.metadata,
                    "status": conversation.status,
                    "last_interaction_at": conversation.last_interaction_at,
                    "updated_at": conversation.updated_at,
                    "created_at": conversation.created_at
                });

                conversations.push(json);
            }
            Err(_) => (),
        };
    }

    match conversations.len() == limit as usize {
        true => {
            conversations.pop();
            match conversations.last() {
                Some(last) => {
                    let pagination_key = base64::encode(last["version_id"].clone().to_string());

                    Ok(serde_json::json!({"conversations": conversations, "pagination_key": pagination_key}))
                }
                None => Ok(serde_json::json!({ "conversations": conversations })),
            }
        }
        false => Ok(serde_json::json!({ "conversations": conversations })),
    }
}
