use crate::{encrypt::encrypt_data, Client, ConversationInfo, ManagerError, Message};
use bson::{doc, Bson, Document};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Messages {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: bson::oid::ObjectId,
    pub client: Client,
    pub interaction_id: bson::oid::ObjectId,
    pub conversation_id: bson::oid::ObjectId,
    pub flow_id: String,
    pub step_id: String,
    pub message_order: i32,
    pub interaction_order: i32,
    pub direction: String, // (SEND, RECEIVE)
    pub payload: String,   // encrypted
    pub content_type: String,
    pub created_at: bson::UtcDateTime,
}

pub fn format_messages(
    data: &ConversationInfo,
    messages: &Vec<Message>,
    interaction_order: i32,
    direction: &str,
) -> Result<Vec<Document>, ManagerError> {
    messages
        .iter()
        .enumerate()
        .map(|(i, var)| {
            format_message(
                data,
                var.clone().message_to_json(),
                i as i32,
                interaction_order,
                direction,
            )
        })
        .collect::<Result<Vec<Document>, ManagerError>>()
}

pub fn format_message(
    data: &ConversationInfo,
    message: serde_json::Value,
    msg_order: i32,
    interaction_order: i32,
    direction: &str,
) -> Result<Document, ManagerError> {
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

pub fn format_event_message(
    data: &ConversationInfo,
    json_event: serde_json::Value,
) -> Result<Document, ManagerError> {
    let event = json_event["payload"].to_owned();
    let time = Bson::UtcDatetime(chrono::Utc::now());

    let doc = doc! {
        "client": bson::to_bson(&data.client)?,
        "interaction_id": data.interaction_id.to_owned(),
        "conversation_id": data.conversation_id.to_owned(),
        "flow_id": data.context.flow.to_owned(),
        "step_id": data.context.step.to_owned(),
        "message_order": 0,
        "interaction_order": 0,
        "direction": "RECEIVE",
        "payload": encrypt_data(&event)?, // encrypted
        "content_type": "event",
        "created_at": time
    };

    Ok(doc)
}

pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    msgs: Vec<Document>,
) -> Result<(), ManagerError> {
    let message = data.db.collection("message");

    message.insert_many(msgs, None)?;

    Ok(())
}