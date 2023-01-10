use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    db_connectors::sqlite::get_db,
    encrypt::{decrypt_data, encrypt_data},
    Client, ConversationInfo, EngineError, SqliteClient,
};

use super::{
    models,
    pagination::*,
    schema::{csml_conversations, csml_messages},
};
use chrono::NaiveDateTime;

pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    msgs: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
    expires_at: Option<NaiveDateTime>,
) -> Result<(), EngineError> {
    if msgs.is_empty() {
        return Ok(());
    }

    let db = get_db(&mut data.db)?;

    let mut new_messages = vec![];
    for (message_order, message) in msgs.iter().enumerate() {
        let conversation_id = models::UUID::parse_str(&data.conversation_id).unwrap();

        let msg = models::NewMessages {
            id: models::UUID::new_v4(),
            conversation_id,

            flow_id: &data.context.flow,
            step_id: data.context.step.get_step_ref(),
            direction,
            payload: encrypt_data(message)?,
            content_type: message["content_type"].as_str().unwrap_or("text"),

            message_order: message_order as i32,
            interaction_order,
            expires_at,
        };

        new_messages.push(msg);
    }

    diesel::insert_into(csml_messages::table)
        .values(&new_messages)
        .execute(db.client.as_mut())?;

    Ok(())
}

pub fn delete_user_messages(client: &Client, db: &mut SqliteClient) -> Result<(), EngineError> {
    let conversations: Vec<models::Conversation> = csml_conversations::table
        .filter(csml_conversations::bot_id.eq(&client.bot_id))
        .filter(csml_conversations::channel_id.eq(&client.channel_id))
        .filter(csml_conversations::user_id.eq(&client.user_id))
        .load(db.client.as_mut())?;

    for conversation in conversations {
        diesel::delete(
            csml_messages::table.filter(csml_messages::conversation_id.eq(&conversation.id)),
        )
        .execute(db.client.as_mut())
        .ok();
    }

    Ok(())
}

pub fn get_client_messages(
    client: &Client,
    db: &mut SqliteClient,
    limit: Option<i64>,
    pagination_key: Option<String>,
    from_date: Option<i64>,
    to_date: Option<i64>,
) -> Result<serde_json::Value, EngineError> {
    let pagination_key = match pagination_key {
        Some(paginate) => paginate.parse::<i64>().unwrap_or(1),
        None => 1,
    };

    let (conversation_with_messages, total_pages) = match from_date {
        Some(from_date) => {
            let from_date = NaiveDateTime::from_timestamp_opt(from_date, 0).unwrap();
            let to_date = match to_date {
                Some(to_date) => NaiveDateTime::from_timestamp_opt(to_date, 0).unwrap(),
                None => chrono::Utc::now().naive_utc(),
            };

            let mut query = csml_conversations::table
                .filter(csml_conversations::bot_id.eq(&client.bot_id))
                .filter(csml_conversations::channel_id.eq(&client.channel_id))
                .filter(csml_conversations::user_id.eq(&client.user_id))
                .inner_join(csml_messages::table)
                .filter(csml_messages::created_at.ge(from_date))
                .filter(csml_messages::created_at.le(to_date))
                .select((csml_conversations::all_columns, csml_messages::all_columns))
                .order_by(csml_messages::created_at.desc())
                .then_order_by(csml_messages::message_order.desc())
                .paginate(pagination_key);

            let limit_per_page = match limit {
                Some(limit) => std::cmp::min(limit, 25),
                None => 25,
            };
            query = query.per_page(limit_per_page);

            query.load_and_count_pages::<(models::Conversation, models::Message)>(db.client.as_mut())?
        }
        None => {
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

            query.load_and_count_pages::<(models::Conversation, models::Message)>(db.client.as_mut())?
        }
    };

    let (_, messages): (Vec<_>, Vec<_>) = conversation_with_messages.into_iter().unzip();

    let mut msgs = vec![];
    for message in messages {
        let json = serde_json::json!({
            "client": {
                "bot_id": &client.bot_id,
                "channel_id": &client.channel_id,
                "user_id": &client.user_id
            },
            "conversation_id": message.conversation_id.get_uuid(),
            "flow_id": message.flow_id,
            "step_id": message.step_id,
            "direction": message.direction,
            "payload": decrypt_data(message.payload)?,

            "updated_at": message.updated_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
            "created_at": message.created_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
        });

        msgs.push(json);
    }

    match pagination_key < total_pages {
        true => {
            let pagination_key = (pagination_key + 1).to_string();
            Ok(serde_json::json!({"messages": msgs, "pagination_key": pagination_key}))
        }
        false => Ok(serde_json::json!({ "messages": msgs })),
    }
}
