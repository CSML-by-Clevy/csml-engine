use crate::{
    db_connectors::mongodb::get_db, encrypt::encrypt_data, ConversationInfo, ManagerError,
};
use bson::{doc, Bson, Document};

fn format_messages(
    data: &ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<Vec<Document>, ManagerError> {
    messages
        .iter()
        .enumerate()
        .map(|(i, var)| format_message(data, var.clone(), i as i32, interaction_order, direction))
        .collect::<Result<Vec<Document>, ManagerError>>()
}

fn format_message(
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

pub fn add_messages_bulk(
    data: &ConversationInfo,
    msgs: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<(), ManagerError> {
    if msgs.len() == 0 {
        return Ok(());
    }
    let docs = format_messages(data, msgs, interaction_order, direction)?;
    let db = get_db(&data.db)?;

    let message = db.collection("message");

    message.insert_many(docs, None)?;

    Ok(())
}
