use crate::{
    db_connectors::{DbMessage ,mongodb::get_db},
    encrypt::{encrypt_data, decrypt_data},
    ConversationInfo, EngineError, Client
};
use bson::{doc, Bson, Document};

fn format_messages(
    data: &ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<Vec<Document>, EngineError> {
    messages
        .iter()
        .enumerate()
        .map(|(i, var)| format_message(data, var.clone(), i as i32, interaction_order, direction))
        .collect::<Result<Vec<Document>, EngineError>>()
}

fn format_message(
    data: &ConversationInfo,
    message: serde_json::Value,
    msg_order: i32,
    interaction_order: i32,
    direction: &str,
) -> Result<Document, EngineError> {
    let time = Bson::UtcDatetime(chrono::Utc::now());
    let doc = doc! {
        "client": bson::to_bson(&data.client)?,
        "interaction_id": &data.interaction_id,
        "conversation_id": &data.conversation_id,
        "flow_id": &data.context.flow,
        "step_id": &data.context.step,
        "message_order": msg_order,
        "interaction_order": interaction_order,
        "direction": direction,
        "payload": encrypt_data(&message)?, // encrypted
        "content_type": "event",
        "created_at": time
    };

    Ok(doc)
}

fn format_message_struct(
    message: bson::ordered::OrderedDocument,
) -> Result<DbMessage, EngineError> {
    let encrypted_payload: String = message.get_str("payload").unwrap().to_owned();
    let payload = decrypt_data(encrypted_payload)?;

    Ok(DbMessage {
        id: message.get_object_id("_id").unwrap().to_hex(), // to_hex bson::oid::ObjectId
        client: bson::from_bson(message.get("client").unwrap().to_owned())?,
        interaction_id: message.get_str("interaction_id").unwrap().to_owned(), 
        conversation_id: message.get_str("conversation_id").unwrap().to_owned(),
        flow_id: message.get_str("flow_id").unwrap().to_owned(),
        step_id: message.get_str("step_id").unwrap().to_owned(),
        message_order: message.get_i32("message_order").unwrap(),
        interaction_order: message.get_i32("interaction_order").unwrap(),
        direction: message.get_str("direction").unwrap().to_owned(),
        payload,
        content_type: message.get_str("content_type").unwrap().to_owned(),
        created_at: message.get_str("created_at").unwrap().to_owned(),
    })
}

pub fn add_messages_bulk(
    data: &ConversationInfo,
    msgs: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<(), EngineError> {
    if msgs.len() == 0 {
        return Ok(());
    }
    let docs = format_messages(data, msgs, interaction_order, direction)?;
    let db = get_db(&data.db)?;

    let message = db.collection("message");

    message.insert_many(docs, None)?;

    Ok(())
}

pub fn delete_user_messages(client: &Client, db: &mongodb::Database) -> Result<(), EngineError> {
    let collection = db.collection("message");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}

pub fn get_conversation_messages(
    client: &Client,
    conversation_id: &str,
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
                "conversation_id": conversation_id,
                "_id": {"$gt": bson::oid::ObjectId::with_string(&key).unwrap() }
            }
        }
        None => doc! {
            "client": bson::to_bson(&client)?,
            "conversation_id": conversation_id,
        },
    };

    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "$natural": -1 })
        .batch_size(30)
        .limit(limit)
        .build();
    let cursor = collection.find(filter, find_options)?;

    let mut messages = vec![];
    for doc in cursor {
        match doc {
            Ok(msg) => {
                let message = format_message_struct(msg)?;

                let json = serde_json::json!({
                    "client": message.client,
                    "interaction_id": message.interaction_id,
                    "conversation_id": message.conversation_id,
                    "flow_id": message.flow_id,
                    "step_id": message.step_id,
                    "direction": message.direction,
                    "payload": message.payload,
                    "content_type": message.content_type,
                    "created_at": message.created_at,
                });

                messages.push(json);
            }
            Err(_) => (),
        };
    }

    match messages.len() == limit as usize {
        true => {
            messages.pop();
            match messages.last() {
                Some(last) => {
                    let pagination_key = base64::encode(last["version_id"].clone().to_string());

                    Ok(serde_json::json!({"messages": messages, "pagination_key": pagination_key}))
                }
                None => Ok(serde_json::json!({ "messages": messages })),
            }
        }
        false => Ok(serde_json::json!({ "messages": messages })),
    }
}