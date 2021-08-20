use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::{insert_into};

use serde_json::Value;

use crate::{
    db_connectors::postgresql::get_db,
    encrypt::{decrypt_data, encrypt_data},
    EngineError, PostgresqlClient,
    ConversationInfo, Memory, Client
};

use super::{
    models,
    schema::memories
};

use std::collections::HashMap;
use std::env;

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &HashMap<String, Memory>,
) -> Result<(), EngineError> {
    if memories.is_empty() {
        return Ok(());
    }

    let db = get_db(&data.db)?;

    let mut new_memories = vec!();
    for (key, mem) in memories.iter() {
        let mem = models::NewMemory {
            client_id: 42,
            key,
            value: encrypt_data(&mem.value)?,
        };

        new_memories.push(mem);
    }

    let instruction: models::Memory = diesel::insert_into(memories::table)
    .values(&new_memories)
    .get_result(&db.client)
    .expect("Error creating memory");

    Ok(())
}

pub fn create_client_memory(
    // client: &Client,
    key: String,
    value: serde_json::Value,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {

    let new_memory = models::NewMemory {
        client_id: 42, // client_id
        key: &key,
        value: encrypt_data(&value)?, // encrypted
    };

    let instruction: models::Memory = diesel::insert_into(memories::table)
    .values(&new_memory)
    .get_result(&db.client)
    .expect("Error creating memory"); 

    Ok(())
}

pub fn get_memories(
    client: &Client,
    db: &PostgresqlClient
) -> Result<serde_json::Value, EngineError> {
    let memories: Vec<models::Memory> = memories::table.filter(memories::client_id.eq(42))
    // .filter(memories::bot_id.eq("Sean"))
    // .filter(memories::channel_id.eq("Sean"))
    // .filter(memories::user_id.eq("Sean"))
    .load(&db.client)
    .expect("Error getting memory"); 

    let mut vec = vec![];
    for mem in memories {
        let value: serde_json::Value = decrypt_data(mem.value)?;
        let mut memory = serde_json::Map::new();

        memory.insert("key".to_owned(), serde_json::json!(mem.key));
        memory.insert("value".to_owned(), value);
        memory.insert("created_at".to_owned(), serde_json::json!(mem.created_at.to_string()));

        vec.push(memory);
    }

    Ok(serde_json::json!(vec))
}

pub fn get_memory(
    client: &Client,
    key: &str,
    db: &PostgresqlClient,
) -> Result<serde_json::Value, EngineError> {

    let mem: models::Memory = memories::table.filter(memories::client_id.eq(42))
    .filter(memories::key.eq(&key))
    // .filter(memories::bot_id.eq("Sean"))
    // .filter(memories::channel_id.eq("Sean"))
    // .filter(memories::user_id.eq("Sean"))
    .get_result(&db.client)
    .expect("Error getting memory"); 

    let mut memory = serde_json::Map::new();
    let value: serde_json::Value = decrypt_data(mem.value)?;

    memory.insert("key".to_owned(), serde_json::json!(mem.key));
    memory.insert("value".to_owned(), value);
    memory.insert("created_at".to_owned(), serde_json::json!(mem.created_at.to_string()));

    Ok(serde_json::json!(memory))
}

pub fn delete_client_memory(
    client: &Client,
    key: &str,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {

    diesel::delete(memories::table
        .filter(memories::client_id.eq(42))
        .filter(memories::key.eq(key))
    ).execute(&db.client);

    Ok(())
}

pub fn delete_client_memories(
    client: &Client,
    db: &PostgresqlClient
) -> Result<(), EngineError> {
    diesel::delete(memories::table
        .filter(memories::client_id.eq(42))
    ).execute(&db.client);

    Ok(())
}
