use crate::{
    db_connectors::mongodb::get_db,
    encrypt::{decrypt_data, encrypt_data},
    Client, ConversationInfo, EngineError, Memory,
};
use bson::{doc, Bson};

fn format_memories(
    data: &mut ConversationInfo,
    memories: &[Memory],
) -> Result<Vec<bson::Document>, EngineError> {
    let client = bson::to_bson(&data.client)?;

    memories
        .iter()
        .fold(Ok(vec![]), |vec, mem | {
            let time = Bson::UtcDatetime(chrono::Utc::now());
            let value = encrypt_data(&mem.value)?;

            let mut vec = vec?;

            vec.push(doc! {
                "client": client.clone(),
                "key": &mem.key,
                "value": value, // encrypted
                "expires_at": Bson::Null,
                "created_at": time.clone(),
                "updated_at": time
            });
            Ok(vec)
        })
}

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &[Memory],
) -> Result<(), EngineError> {
    if memories.is_empty() {
        return Ok(());
    }

    let mem = format_memories(data, memories)?;
    let db = get_db(&data.db)?;

    let collection = db.collection("memory");
    collection.insert_many(mem, None)?;

    Ok(())
}

pub fn create_client_memory(
    client: &Client,
    key: String,
    value: serde_json::Value,
    db: &mongodb::Database,
) -> Result<(), EngineError> {
    let memory = doc! {
        "client": bson::to_bson(&client)?,
        "key": key,
        "value": encrypt_data(&value)?, // encrypted
        "expires_at": Bson::Null,
        "created_at": Bson::UtcDatetime(chrono::Utc::now())
    };

    let collection = db.collection("memory");
    collection.insert_one(memory, None)?;

    Ok(())
}

pub fn get_memories(
    client: &Client,
    db: &mongodb::Database,
) -> Result<serde_json::Value, EngineError> {
    let collection = db.collection("memory");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
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

pub fn delete_client_memory(client: &Client, key: &str, db: &mongodb::Database) -> Result<(), EngineError> {
    let collection = db.collection("memory");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
        "key": key,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}

pub fn delete_client_memories(client: &Client, db: &mongodb::Database) -> Result<(), EngineError> {
    let collection = db.collection("memory");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}