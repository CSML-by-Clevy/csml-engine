use crate::{
    db_connectors::{mongodb::get_db, DbMessage},
    encrypt::{decrypt_data, encrypt_data},
    Client, ConversationInfo, EngineError, MongoDbClient,
};
use bson::{doc, Document};
use chrono::SecondsFormat;

fn format_messages(
    data: &ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
    expires_at: Option<bson::DateTime>,
) -> Result<Vec<Document>, EngineError> {
    messages
        .iter()
        .enumerate()
        .map(|(i, var)| {
            format_message(
                data,
                var.clone(),
                i as i32,
                interaction_order,
                direction,
                expires_at,
            )
        })
        .collect::<Result<Vec<Document>, EngineError>>()
}

fn format_message(
    data: &ConversationInfo,
    message: serde_json::Value,
    msg_order: i32,
    interaction_order: i32,
    direction: &str,
    expires_at: Option<bson::DateTime>,
) -> Result<Document, EngineError> {
    let time = bson::DateTime::from_chrono(chrono::Utc::now());
    let doc = doc! {
        "client": bson::to_bson(&data.client)?,
        "conversation_id": &data.conversation_id,
        "flow_id": &data.context.flow,
        "step_id": &data.context.step.get_step(),
        "message_order": msg_order,
        "interaction_order": interaction_order,
        "direction": direction,
        "payload": encrypt_data(&message)?, // encrypted
        "expires_at": expires_at,
        "created_at": time
    };

    Ok(doc)
}

fn format_message_struct(message: bson::document::Document) -> Result<DbMessage, EngineError> {
    let encrypted_payload: String = message.get_str("payload").unwrap().to_owned();
    let payload = decrypt_data(encrypted_payload)?;

    Ok(DbMessage {
        id: message.get_object_id("_id").unwrap().to_hex(), // to_hex bson::oid::ObjectId
        client: bson::from_bson(message.get("client").unwrap().to_owned())?,
        conversation_id: message.get_str("conversation_id").unwrap().to_owned(),
        flow_id: message.get_str("flow_id").unwrap().to_owned(),
        step_id: message.get_str("step_id").unwrap().to_owned(),
        message_order: message.get_i32("message_order").unwrap(),
        interaction_order: message.get_i32("interaction_order").unwrap(),
        direction: message.get_str("direction").unwrap().to_owned(),
        payload,
        created_at: message
            .get_datetime("created_at")
            .unwrap()
            .to_chrono()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
    })
}

pub fn add_messages_bulk(
    data: &ConversationInfo,
    msgs: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
    expires_at: Option<bson::DateTime>,
) -> Result<(), EngineError> {
    if msgs.len() == 0 {
        return Ok(());
    }
    let docs = format_messages(data, msgs, interaction_order, direction, expires_at)?;
    let db = get_db(&data.db)?;

    let message = db.client.collection::<Document>("message");

    message.insert_many(docs, None)?;

    Ok(())
}

pub fn delete_user_messages(client: &Client, db: &MongoDbClient) -> Result<(), EngineError> {
    let collection = db.client.collection::<Document>("message");

    let filter = doc! {
        "client.bot_id": client.bot_id.to_owned(),
        "client.user_id": client.user_id.to_owned(),
        "client.channel_id": client.channel_id.to_owned(),
    };

    collection.delete_many(filter, None)?;

    Ok(())
}

pub fn get_client_messages(
    client: &Client,
    db: &MongoDbClient,
    limit: Option<i64>,
    pagination_key: Option<String>,
    from_date: Option<i64>,
    to_date: Option<i64>,
) -> Result<serde_json::Value, EngineError> {
    let collection = db.client.collection::<Document>("message");

    let limit = match limit {
        Some(limit) => std::cmp::min(limit + 1, 26),
        None => 26,
    };

    let filter = match (pagination_key, from_date) {
        (Some(key), Some(from_date)) => {
            let from_date = bson::DateTime::from_millis(from_date * 1000);
            let to_date = match to_date {
                Some(to_date) => bson::DateTime::from_millis(to_date * 1000),
                None => bson::DateTime::from_chrono(chrono::Utc::now()),
            };

            doc! {
                "client.bot_id": client.bot_id.to_owned(),
                "client.user_id": client.user_id.to_owned(),
                "client.channel_id": client.channel_id.to_owned(),
                "_id": {"$gt": bson::oid::ObjectId::parse_str(&key).unwrap() },
                "created_at": {"$gte": from_date, "$lt": to_date}
            }
        }
        (None, Some(from_date)) => {
            let from_date = bson::DateTime::from_millis(from_date * 1000);
            let to_date = match to_date {
                Some(to_date) => bson::DateTime::from_millis(to_date * 1000),
                None => bson::DateTime::from_chrono(chrono::Utc::now()),
            };

            doc! {
                "client.bot_id": client.bot_id.to_owned(),
                "client.user_id": client.user_id.to_owned(),
                "client.channel_id": client.channel_id.to_owned(),
                "created_at": {"$gte": from_date, "$lt": to_date}
            }
        }
        (Some(key), None) => {
            doc! {
                "client.bot_id": client.bot_id.to_owned(),
                "client.user_id": client.user_id.to_owned(),
                "client.channel_id": client.channel_id.to_owned(),
                "_id": {"$gt": bson::oid::ObjectId::parse_str(&key).unwrap() },
            }
        }
        (None, None) => doc! {
            "client.bot_id": client.bot_id.to_owned(),
            "client.user_id": client.user_id.to_owned(),
            "client.channel_id": client.channel_id.to_owned(),
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
                    "conversation_id": message.conversation_id,
                    "flow_id": message.flow_id,
                    "step_id": message.step_id,
                    "direction": message.direction,
                    "payload": message.payload,
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
                    let pagination_key = base64::engine::general_purpose::STANDARD.encode(last["version_id"].clone().to_string());

                    Ok(serde_json::json!({"messages": messages, "pagination_key": pagination_key}))
                }
                None => Ok(serde_json::json!({ "messages": messages })),
            }
        }
        false => Ok(serde_json::json!({ "messages": messages })),
    }
}
