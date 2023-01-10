use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{Client, DbConversation, EngineError, SqliteClient};
use chrono::NaiveDateTime;

use super::{models, pagination::*, schema::csml_conversations};

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    expires_at: Option<NaiveDateTime>,
    db: &mut SqliteClient,
) -> Result<String, EngineError> {
    let id = models::UUID::new_v4();

    let new_conversation = models::NewConversation {
        id,
        bot_id: &client.bot_id,
        channel_id: &client.channel_id,
        user_id: &client.user_id,
        flow_id,
        step_id,
        status: "OPEN",
        expires_at,
    };

    diesel::insert_into(csml_conversations::table)
        .values(&new_conversation)
        .execute(db.client.as_mut())?;

    Ok(id.to_string())
}

pub fn close_conversation(
    id: &str,
    _client: &Client,
    status: &str,
    db: &mut SqliteClient,
) -> Result<(), EngineError> {
    let id = models::UUID::parse_str(id).unwrap();

    diesel::update(csml_conversations::table.filter(csml_conversations::id.eq(id)))
        .set(csml_conversations::status.eq(status))
        .execute(db.client.as_mut())?;

    Ok(())
}

pub fn close_all_conversations(client: &Client, db: &mut SqliteClient) -> Result<(), EngineError> {
    diesel::update(
        csml_conversations::table
            .filter(csml_conversations::bot_id.eq(&client.bot_id))
            .filter(csml_conversations::channel_id.eq(&client.channel_id))
            .filter(csml_conversations::user_id.eq(&client.user_id)),
    )
    .set(csml_conversations::status.eq("CLOSED"))
    .execute(db.client.as_mut())?;

    Ok(())
}

pub fn get_latest_open(
    client: &Client,
    db: &mut SqliteClient,
) -> Result<Option<DbConversation>, EngineError> {
    let result: Result<models::Conversation, diesel::result::Error> = csml_conversations::table
        .filter(csml_conversations::bot_id.eq(&client.bot_id))
        .filter(csml_conversations::channel_id.eq(&client.channel_id))
        .filter(csml_conversations::user_id.eq(&client.user_id))
        .filter(csml_conversations::status.eq("OPEN"))
        .order_by(csml_conversations::updated_at.desc())
        .limit(1)
        .get_result(db.client.as_mut());

    match result {
        Ok(conv) => {
            let conversation = DbConversation {
                id: conv.id.to_string(),
                client: Client {
                    bot_id: conv.bot_id,
                    channel_id: conv.channel_id,
                    user_id: conv.user_id,
                },
                flow_id: conv.flow_id,
                step_id: conv.step_id,
                status: conv.status,
                last_interaction_at: conv
                    .last_interaction_at
                    .format("%Y-%m-%dT%H:%M:%S%.fZ")
                    .to_string(),
                updated_at: conv.updated_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
                created_at: conv.created_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
            };

            Ok(Some(conversation))
        }
        Err(..) => Ok(None),
    }
}

pub fn update_conversation(
    conversation_id: &str,
    flow_id: Option<String>,
    step_id: Option<String>,
    db: &mut SqliteClient,
) -> Result<(), EngineError> {
    let id = models::UUID::parse_str(conversation_id).unwrap();

    match (flow_id, step_id) {
        (Some(flow_id), Some(step_id)) => {
            diesel::update(csml_conversations::table.filter(csml_conversations::id.eq(&id)))
                .set((
                    csml_conversations::flow_id.eq(flow_id.as_str()),
                    csml_conversations::step_id.eq(step_id.as_str()),
                ))
                .execute(db.client.as_mut())?;
        }
        (Some(flow_id), _) => {
            diesel::update(csml_conversations::table.filter(csml_conversations::id.eq(&id)))
                .set(csml_conversations::flow_id.eq(flow_id.as_str()))
                .execute(db.client.as_mut())?;
        }
        (_, Some(step_id)) => {
            diesel::update(csml_conversations::table.filter(csml_conversations::id.eq(&id)))
                .set(csml_conversations::step_id.eq(step_id.as_str()))
                .execute(db.client.as_mut())?;
        }
        _ => return Ok(()),
    };

    Ok(())
}

pub fn delete_user_conversations(
    client: &Client,
    db: &mut SqliteClient,
) -> Result<(), EngineError> {
    diesel::delete(
        csml_conversations::table
            .filter(csml_conversations::bot_id.eq(&client.bot_id))
            .filter(csml_conversations::channel_id.eq(&client.channel_id))
            .filter(csml_conversations::user_id.eq(&client.user_id)),
    )
    .execute(db.client.as_mut())
    .ok();

    Ok(())
}

pub fn get_client_conversations(
    client: &Client,
    db: &mut SqliteClient,
    limit: Option<i64>,
    pagination_key: Option<String>,
) -> Result<serde_json::Value, EngineError> {
    let pagination_key = match pagination_key {
        Some(paginate) => paginate.parse::<i64>().unwrap_or(1),
        None => 1,
    };

    let mut query = csml_conversations::table
        .order_by(csml_conversations::updated_at.desc())
        .filter(csml_conversations::bot_id.eq(&client.bot_id))
        .filter(csml_conversations::channel_id.eq(&client.channel_id))
        .filter(csml_conversations::user_id.eq(&client.user_id))
        .paginate(pagination_key);

    let limit_per_page = match limit {
        Some(limit) => std::cmp::min(limit, 25),
        None => 25,
    };
    query = query.per_page(limit_per_page);

    let (conversations, total_pages) =
        query.load_and_count_pages::<models::Conversation>(db.client.as_mut())?;

    let mut convs = vec![];
    for conversation in conversations {
        let json = serde_json::json!({
            "client": {
                "bot_id": conversation.bot_id,
                "channel_id": conversation.channel_id,
                "user_id": conversation.user_id
            },
            "flow_id": conversation.flow_id,
            "step_id": conversation.step_id,
            "status": conversation.status,
            "last_interaction_at": conversation.last_interaction_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
            "updated_at": conversation.updated_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
            "created_at": conversation.created_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
        });

        convs.push(json);
    }

    match pagination_key < total_pages {
        true => {
            let pagination_key = (pagination_key + 1).to_string();
            Ok(serde_json::json!({"conversations": convs, "pagination_key": pagination_key}))
        }
        false => Ok(serde_json::json!({ "conversations": convs })),
    }
}

pub fn delete_all_bot_data(bot_id: &str, db: &mut SqliteClient) -> Result<(), EngineError> {
    diesel::delete(csml_conversations::table.filter(csml_conversations::bot_id.eq(bot_id)))
        .execute(db.client.as_mut())
        .ok();

    Ok(())
}
