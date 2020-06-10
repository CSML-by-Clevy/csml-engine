use crate::{Client, ConversationInfo, ManagerError, Message};

pub fn format_messages(
    data: &ConversationInfo,
    messages: &Vec<Message>,
    interaction_order: i32,
    direction: &str,
) -> Result<Vec<i32>, ManagerError> { // Document
    // messages
    //     .iter()
    //     .enumerate()
    //     .map(|(i, var)| {
    //         format_message(
    //             data,
    //             var.clone().message_to_json(),
    //             i as i32,
    //             interaction_order,
    //             direction,
    //         )
    //     })
    //     .collect::<Result<Vec<Document>, ManagerError>>()
    unimplemented!()
}

pub fn format_message(
    data: &ConversationInfo,
    message: serde_json::Value,
    msg_order: i32,
    interaction_order: i32,
    direction: &str,
) -> Result<i32, ManagerError> { //Document
    // let time = Bson::UtcDatetime(chrono::Utc::now());
    // let doc = doc! {
    //     "client": bson::to_bson(&data.client)?,
    //     "interaction_id": &data.interaction_id,
    //     "conversation_id": &data.conversation_id,
    //     "flow_id": &data.context.flow,
    //     "step_id": &data.context.step,
    //     "message_order": msg_order,
    //     "interaction_order": interaction_order,
    //     "direction": direction,
    //     "payload": encrypt_data(&message)?, // encrypted
    //     "content_type": "event",
    //     "created_at": time
    // };
    unimplemented!()
    // Ok(doc)
}

pub fn format_event_message(
    data: &ConversationInfo,
    json_event: serde_json::Value,
) -> Result<i32, ManagerError> { //Document
    // let event = json_event["payload"].to_owned();
    // let time = Bson::UtcDatetime(chrono::Utc::now());

    // let doc = doc! {
    //     "client": bson::to_bson(&data.client)?,
    //     "interaction_id": data.interaction_id.to_owned(),
    //     "conversation_id": data.conversation_id.to_owned(),
    //     "flow_id": data.context.flow.to_owned(),
    //     "step_id": data.context.step.to_owned(),
    //     "message_order": 0,
    //     "interaction_order": 0,
    //     "direction": "RECEIVE",
    //     "payload": encrypt_data(&event)?, // encrypted
    //     "content_type": "event",
    //     "created_at": time
    // };
    unimplemented!()
    // Ok(doc)
}

pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    msgs: Vec<i32>, // Document
) -> Result<(), ManagerError> {
    unimplemented!()
    // Ok(())
}
