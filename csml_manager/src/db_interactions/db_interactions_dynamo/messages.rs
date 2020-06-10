use crate::{ConversationInfo, Message};
use dynamodb::{
    apis::Error,
    models::{create_message_body::Direction, CreateMessageBody},
};
// use serde_json::Value;
use uuid::Uuid;

pub fn format_messages(
    data: &ConversationInfo,
    messages: &Vec<Message>,
    interaction_order: i32,
    direction: Direction,
) -> Vec<CreateMessageBody> {
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
        .collect::<Vec<_>>()
}

pub fn format_message(
    data: &ConversationInfo,
    message: serde_json::Value,
    msg_order: i32,
    interaction_order: i32,
    direction: Direction,
) -> CreateMessageBody {
    CreateMessageBody::new(
        Uuid::new_v4().to_string(),
        data.interaction_id.to_owned(),
        msg_order,
        interaction_order,
        data.flow_info.flow.id.to_owned(),
        data.flow_info.step_id.to_owned(),
        direction,
        message,
    )
}

pub fn format_event_message(
    data: &ConversationInfo,
    json_event: serde_json::Value,
) -> CreateMessageBody {
    let event = json_event["payload"].to_owned();
    CreateMessageBody::new(
        Uuid::new_v4().to_string(),
        data.interaction_id.to_owned(),
        0,
        0,
        data.flow_info.flow.id.to_owned(),
        data.flow_info.step_id.to_owned(),
        Direction::RECEIVE,
        event,
    )
}

// pub fn add_message(
//     conversation_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     create_message_body: crate::models::CreateMessageBody
// )

// pub fn add_message(
//     data: &mut ConversationInfo,
//     msg: CreateMessageBody,
// ) -> Result<(), Error> {
//     data.api_client.messages_api().add_message(
//         &data.conversation_id,
//         &data.context.client.bot_id,
//         &data.context.client.user_id,
//         &data.context.client.channel_id,
//         msg,
//     )
// }

// pub fn add_messages_bulk(
//     conversation_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     create_message_body: Vec<crate::models::CreateMessageBody>
// )
pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    msgs: Vec<CreateMessageBody>,
) -> Result<(), Error> {
    data.api_client.messages_api().add_messages_bulk(
        &data.conversation_id,
        &data.client.bot_id,
        &data.client.user_id,
        &data.client.channel_id,
        msgs,
    )
}
