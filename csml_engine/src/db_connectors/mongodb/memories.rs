use crate::{
    db_connectors::mongodb::get_db,
    encrypt::{decrypt_data, encrypt_data},
    Client, ConversationInfo, EngineError, Memory, MongoDbClient,
};
use bson::{doc, Bson, Document};
use std::collections::HashMap;

fn format_memories(
    data: &mut ConversationInfo,
    memories: &HashMap<String, Memory>,
    expires_at: Option<bson::DateTime>,
) -> Result<Vec<bson::Document>, EngineError> {
    let client = bson::to_bson(&data.client)?;

    memories.iter().fold(Ok(vec![]), |vec, (_, mem)| {
        let time = bson::DateTime::from_chrono(chrono::Utc::now());
        let value = encrypt_data(&mem.value)?;

        let mut vec = vec?;

        vec.push(doc! {
            "client": client.clone(),
            "key": &mem.key,
            "value": value, // encrypted
            "expires_at": Bson::Null,
            "expires_at": expires_at,
            "created_at": time.clone(),
            "updated_at": time
        });
        Ok(vec)
    })
}

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &HashMap<String, Memory>,
    expires_at: Option<bson::DateTime>,
) -> Result<(), EngineError> {
    if memories.is_empty() {
        return Ok(());
    }

    let mem = format_memories(data, memories, expires_at)?;
    let db = get_db(&data.db)?;

    let collection = db.client.collection::<Document>("memory");
    collection.insert_many(mem, None)?;

    Ok(())
}

pub fn create_client_memory(
    client: &Client,
    key: String,
    value: serde_json::Value,
    expires_at: Option<bson::DateTime>,
    db: &MongoDbClient,
) -> Result<(), EngineError> {
    let time = bson::DateTime::from_chrono(chrono::Utc::now());
    let memory = doc! {
        "client": bson::to_bson(&client)?,
        "key": key,
        "value": encrypt_data(&value)?, // encrypted
        "expires_at": expires_at,
        "created_at": &time,
        "updated_at": time
    };

    let collection = db.client.collection::<Document>("memory");
    collection.insert_one(memory, None)?;

    Ok(())
}

pub fn internal_use_get_memories(
    client: &Client,
    db: &MongoDbClient,
) -> Result<serde_json::Value, EngineError> {
    let collection = db.client.collection::<Document>("memory");

    let filter = doc! {
        "client.bot_id": client.bot_id.to_owned(),
        "client.user_id": client.user_id.to_owned(),
        "client.channel_id": client.channel_id.to_owned(),
    };
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "$natural": -1 })
        .build();

    let cursor = collection.find(filter, find_options)?;
    let mut map = serde_json::Map::new();

    for elem in cursor {
        if let Ok(doc) = elem {
            let mem: serde_json::Value = bson::from_bson(bson::Bson::Document(doc))?;
            let value: serde_json::Value = decrypt_data(mem["value"].as_str().unwrap().to_owned())?;

            if !map.contains_key(mem["key"].as_str().unwrap()) {
                map.insert(mem["key"].as_str().unwrap().to_owned(), value);
            }
        }
    }

    Ok(serde_json::json!(map))
}

pub fn get_memories(client: &Client, db: &MongoDbClient) -> Result<serde_json::Value, EngineError> {
    let collection = db.client.collection::<Document>("memory");

    let filter = doc! {
        "client.bot_id": client.bot_id.to_owned(),
        "client.user_id": client.user_id.to_owned(),
        "client.channel_id": client.channel_id.to_owned(),
    };
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "$natural": -1 })
        .build();

    let cursor = collection.find(filter, find_options)?;

    let mut vec = vec![];
    for elem in cursor {
        if let Ok(doc) = elem {
            let mem: serde_json::Value = bson::from_bson(bson::Bson::Document(doc))?;
            let value: serde_json::Value = decrypt_data(mem["value"].as_str().unwrap().to_owned())?;
            let mut memory = serde_json::Map::new();

            memory.insert("key".to_owned(), mem["key"].clone());
            memory.insert("value".to_owned(), value);
            memory.insert("created_at".to_owned(), mem["created_at"]["$date"].clone());

            vec.push(memory);
        }
    }

    Ok(serde_json::json!(vec))
}

pub fn get_memory(
    client: &Client,
    key: &str,
    db: &MongoDbClient,
) -> Result<serde_json::Value, EngineError> {
    let collection = db.client.collection::<Document>("memory");

    let filter = doc! {
        "client.bot_id": client.bot_id.to_owned(),
        "client.user_id": client.user_id.to_owned(),
        "client.channel_id": client.channel_id.to_owned(),
        "key": key,
    };
    let find_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "$natural": -1 })
        .build();

    let result = collection.find_one(filter, find_options)?;

    if let Some(doc) = result {
        let mem: serde_json::Value = bson::from_bson(bson::Bson::Document(doc))?;
        let mut memory = serde_json::Map::new();

        memory.insert("key".to_owned(), mem["key"].clone());
        memory.insert(
            "value".to_owned(),
            decrypt_data(mem["value"].as_str().unwrap().to_owned())?,
        );
        memory.insert("created_at".to_owned(), mem["created_at"]["$date"].clone());

        return Ok(serde_json::json!(memory));
    } else {
        return Ok(serde_json::Value::Null);
    }
}

pub fn delete_client_memory(
    client: &Client,
    key: &str,
    db: &MongoDbClient,
) -> Result<(), EngineError> {
    let collection = db.client.collection::<Document>("memory");

    let filter = doc! {
        "client.bot_id": client.bot_id.to_owned(),
        "client.user_id": client.user_id.to_owned(),
        "client.channel_id": client.channel_id.to_owned(),
        "key": key,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}

pub fn delete_client_memories(client: &Client, db: &MongoDbClient) -> Result<(), EngineError> {
    let collection = db.client.collection::<Document>("memory");

    let filter = doc! {
        "client.bot_id": client.bot_id.to_owned(),
        "client.user_id": client.user_id.to_owned(),
        "client.channel_id": client.channel_id.to_owned(),
    };

    collection.delete_many(filter, None)?;

    Ok(())
}
