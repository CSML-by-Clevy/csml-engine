use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};

use crate::{
    db_connectors::postgresql::get_db,
    encrypt::{encrypt_data, decrypt_data},
    EngineError, PostgresqlClient,
    ConversationInfo, Client
};

use super::{
    models,
    schema::{
        csml_messages, csml_conversations
    },
    pagination::*
};

pub fn add_messages_bulk(
    data: &ConversationInfo,
    msgs: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<(), EngineError> {
    if msgs.len() == 0 {
        return Ok(());
    }

    let db = get_db(&data.db)?;

    let mut new_messages = vec!();
    for (message_order, message) in msgs.iter().enumerate() {

        let interaction_id = uuid::Uuid::parse_str(&data.interaction_id).unwrap();
        let conversation_id = uuid::Uuid::parse_str(&data.conversation_id).unwrap();

        let msg = models::NewMessages {
            id: uuid::Uuid::new_v4(),

            interaction_id,
            conversation_id,

            flow_id: &data.context.flow,
            step_id: &data.context.step,
            direction,
            payload: encrypt_data(&message)?,
            content_type: &message["content_type"].as_str().unwrap_or("text"),
            message_order: message_order as i32,
            interaction_order,
        };

        new_messages.push(msg);
    }

    diesel::insert_into(csml_messages::table)
    .values(&new_messages)
    .get_result::<models::Message>(&db.client)?;

    Ok(())
}

pub fn delete_user_messages(
    client: &Client,
    db: &PostgresqlClient
) -> Result<(), EngineError> {

    let conversations: Vec<models::Conversation> = csml_conversations::table
        .filter(csml_conversations::bot_id.eq(&client.bot_id))
        .filter(csml_conversations::channel_id.eq(&client.channel_id))
        .filter(csml_conversations::user_id.eq(&client.user_id))
        .load(&db.client)?;

    for conversation in conversations {
        diesel::delete(
            csml_messages::table
            .filter(csml_messages::conversation_id.eq(&conversation.id))
        ).execute(&db.client).ok();
    }

    Ok(())
}

pub fn get_client_messages(
    client: &Client,
    db: &PostgresqlClient,
    limit: Option<i64>,
    pagination_key: Option<String>,
) -> Result<serde_json::Value, EngineError> {
    let pagination_key = match pagination_key {
        Some(paginate) => paginate.parse::<i64>().unwrap_or(1),
        None => 1
    };

    let mut query = csml_conversations::table
        .filter(csml_conversations::bot_id.eq(&client.bot_id))
        .filter(csml_conversations::channel_id.eq(&client.channel_id))
        .filter(csml_conversations::user_id.eq(&client.user_id))
        .inner_join(csml_messages::table)
        .select((csml_conversations::all_columns, csml_messages::all_columns))
        .order_by(csml_messages::created_at.desc())
        .then_order_by(csml_messages::message_order.desc())
        .paginate(pagination_key);

    let limit_per_page = match limit {
        Some(limit) => std::cmp::min(limit, 25),
        None => 25,
    };
    query = query.per_page(limit_per_page);

    let (conversation_with_messages, total_pages) =
        query.load_and_count_pages::<(models::Conversation, models::Message)>(&db.client)?;
    let (_, messages): (Vec<_>, Vec<_>) = conversation_with_messages.into_iter().unzip();

    let mut msgs = vec![];
    for message in messages {
        let json = serde_json::json!({
            "client": { 
                "bot_id": &client.bot_id,
                "channel_id": &client.channel_id,
                "user_id": &client.user_id
            },
            "interaction_id": message.interaction_id,
            "conversation_id": message.conversation_id,
            "flow_id": message.flow_id,
            "step_id": message.step_id,
            "direction": message.direction,
            "payload": decrypt_data(message.payload)?,
            "content_type": message.content_type,

            "updated_at": message.updated_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
            "created_at": message.created_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
        });

        msgs.push(json);
    }

    match pagination_key < total_pages {
        true => {
            let pagination_key = (pagination_key + 1).to_string();
            Ok(
                serde_json::json!({"messages": msgs, "pagination_key": pagination_key}),
            )
        }
        false => Ok(serde_json::json!({ "messages": msgs })),
    }
}
