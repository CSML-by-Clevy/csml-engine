use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    db_connectors::postgresql::get_db,
    encrypt::{decrypt_data, encrypt_data},
    Client, ConversationInfo, EngineError, Memory, PostgresqlClient,
};

use super::{models, schema::csml_memories};

use chrono::NaiveDateTime;
use std::collections::HashMap;

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &HashMap<String, Memory>,
    expires_at: Option<NaiveDateTime>,
) -> Result<(), EngineError> {
    if memories.is_empty() {
        return Ok(());
    }

    let db = get_db(&mut data.db)?;

    for (key, mem) in memories.iter() {
        create_client_memory(&data.client, key, &mem.value, expires_at, db)?;
    }

    Ok(())
}

pub fn create_client_memory(
    client: &Client,
    key: &str,
    value: &serde_json::Value,
    expires_at: Option<NaiveDateTime>,
    db: &mut PostgresqlClient,
) -> Result<(), EngineError> {
    let value = encrypt_data(value)?;

    let new_memories = models::NewMemory {
        id: uuid::Uuid::new_v4(),
        bot_id: &client.bot_id,
        channel_id: &client.channel_id,
        user_id: &client.user_id,
        key,
        value: value.clone(),
        expires_at,
    };

    diesel::insert_into(csml_memories::table)
        .values(&new_memories)
        .on_conflict((
            csml_memories::bot_id,
            csml_memories::channel_id,
            csml_memories::user_id,
            csml_memories::key,
        ))
        .do_update()
        .set(csml_memories::value.eq(value))
        .execute(db.client.as_mut())?;

    Ok(())
}

pub fn internal_use_get_memories(
    client: &Client,
    db: &mut PostgresqlClient,
) -> Result<serde_json::Value, EngineError> {
    let memories: Vec<models::Memory> = csml_memories::table
        .filter(csml_memories::bot_id.eq(&client.bot_id))
        .filter(csml_memories::channel_id.eq(&client.channel_id))
        .filter(csml_memories::user_id.eq(&client.user_id))
        .load(db.client.as_mut())?;

    let mut map = serde_json::Map::new();
    for mem in memories {
        if !map.contains_key(&mem.key) {
            let value: serde_json::Value = decrypt_data(mem.value)?;
            map.insert(mem.key, value);
        }
    }

    Ok(serde_json::json!(map))
}

pub fn get_memories(
    client: &Client,
    db: &mut PostgresqlClient,
) -> Result<serde_json::Value, EngineError> {
    let memories: Vec<models::Memory> = csml_memories::table
        .filter(csml_memories::bot_id.eq(&client.bot_id))
        .filter(csml_memories::channel_id.eq(&client.channel_id))
        .filter(csml_memories::user_id.eq(&client.user_id))
        .load(db.client.as_mut())?;

    let mut vec = vec![];
    for mem in memories {
        let value: serde_json::Value = decrypt_data(mem.value)?;
        let mut memory = serde_json::Map::new();

        memory.insert("key".to_owned(), serde_json::json!(mem.key));
        memory.insert("value".to_owned(), value);
        memory.insert(
            "created_at".to_owned(),
            serde_json::json!(mem.created_at.to_string()),
        );

        vec.push(memory);
    }

    Ok(serde_json::json!(vec))
}

pub fn get_memory(
    client: &Client,
    key: &str,
    db: &mut PostgresqlClient,
) -> Result<serde_json::Value, EngineError> {
    let mem: models::Memory = csml_memories::table
        .filter(csml_memories::key.eq(&key))
        .filter(csml_memories::bot_id.eq(&client.bot_id))
        .filter(csml_memories::channel_id.eq(&client.channel_id))
        .filter(csml_memories::user_id.eq(&client.user_id))
        .get_result(db.client.as_mut())?;

    let mut memory = serde_json::Map::new();
    let value: serde_json::Value = decrypt_data(mem.value)?;

    memory.insert("key".to_owned(), serde_json::json!(mem.key));
    memory.insert("value".to_owned(), value);
    memory.insert(
        "created_at".to_owned(),
        serde_json::json!(mem.created_at.to_string()),
    );

    Ok(serde_json::json!(memory))
}

pub fn delete_client_memory(
    client: &Client,
    key: &str,
    db: &mut PostgresqlClient,
) -> Result<(), EngineError> {
    diesel::delete(
        csml_memories::table
            .filter(csml_memories::bot_id.eq(&client.bot_id))
            .filter(csml_memories::channel_id.eq(&client.channel_id))
            .filter(csml_memories::user_id.eq(&client.user_id))
            .filter(csml_memories::key.eq(key)),
    )
    .execute(db.client.as_mut())
    .ok();

    Ok(())
}

pub fn delete_client_memories(
    client: &Client,
    db: &mut PostgresqlClient,
) -> Result<(), EngineError> {
    diesel::delete(
        csml_memories::table
            .filter(csml_memories::bot_id.eq(&client.bot_id))
            .filter(csml_memories::channel_id.eq(&client.channel_id))
            .filter(csml_memories::user_id.eq(&client.user_id)),
    )
    .execute(db.client.as_mut())
    .ok();

    Ok(())
}

pub fn delete_all_bot_data(bot_id: &str, db: &mut PostgresqlClient) -> Result<(), EngineError> {
    diesel::delete(csml_memories::table.filter(csml_memories::bot_id.eq(bot_id)))
        .execute(db.client.as_mut())
        .ok();

    Ok(())
}
