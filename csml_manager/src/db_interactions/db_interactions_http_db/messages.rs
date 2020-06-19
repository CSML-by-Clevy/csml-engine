use crate::{db_interactions::db_interactions_http_db::get_db, ConversationInfo, ManagerError};
use http_db::models::{create_message_body::Direction, CreateMessageBody};
use uuid::Uuid;

fn get_direction(status: &str) -> Direction {
    match status {
        send if send == "SEND" => Direction::SEND,
        receive if receive == "RECEIVE" => Direction::RECEIVE,
        _ => unreachable!(),
    }
}

fn format_messages(
    data: &ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: Direction,
) -> Vec<CreateMessageBody> {
    messages
        .iter()
        .enumerate()
        .map(|(i, var)| format_message(data, var.clone(), i as i32, interaction_order, direction))
        .collect::<Vec<_>>()
}

fn format_message(
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
        "".to_owned(), // data.flow_info.flow.id.to_owned(),
        "".to_owned(), // data.flow_info.step_id.to_owned(),
        direction,
        message,
    )
}

pub fn add_messages_bulk(
    data: &ConversationInfo,
    msgs: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<(), ManagerError> {
    let msg_body = format_messages(data, msgs, interaction_order, get_direction(direction));

    let db = get_db(&data.db)?;

    db.messages_api().add_messages_bulk(
        &data.conversation_id,
        &data.client.bot_id,
        &data.client.user_id,
        &data.client.channel_id,
        msg_body,
    )?;

    Ok(())
}
