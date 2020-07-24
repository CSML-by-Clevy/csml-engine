use crate::{
    db_interactions::db_interactions_mongo::get_db,
    encrypt::{decrypt_data, encrypt_data},
    Client, ContextJson, ConversationInfo, ManagerError, Memories,
};
use bson::{doc, Bson};

fn format_memories(
    data: &mut ConversationInfo,
    memories: &[Memories],
    interaction_order: i32,
) -> Result<Vec<bson::Document>, ManagerError> {
    let client = bson::to_bson(&data.client)?;

    memories
        .iter()
        .enumerate()
        .fold(Ok(vec![]), |vec, (memorie_order, var)| {
            let time = Bson::UtcDatetime(chrono::Utc::now());
            let value = encrypt_data(&var.value)?;

            let mut vec = vec?;

            vec.push(doc! {
                "client": client.clone(),
                "interaction_id": &data.interaction_id,
                "conversation_id": &data.conversation_id,
                "flow_id": &data.context.flow,
                "step_id": &data.context.step,
                "memory_order": memorie_order as i32,
                "interaction_order": interaction_order,
                "key": &var.key,
                "value": value, // encrypted
                "expires_at": Bson::Null,
                "created_at": time
            });
            Ok(vec)
        })
}

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &[Memories],
    interaction_order: i32,
) -> Result<(), ManagerError> {
    if memories.len() == 0 {
        return Ok(());
    }
    let mem = format_memories(data, memories, interaction_order)?;
    let db = get_db(&data.db)?;

    let collection = db.collection("memory");
    collection.insert_many(mem, None)?;

    Ok(())
}

pub fn get_memories(
    client: &Client,
    context: &mut ContextJson,
    metadata: &serde_json::Value,
    db: &mongodb::Database,
) -> Result<(), ManagerError> {
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
            let memorie: serde_json::Value = bson::from_bson(bson::Bson::Document(doc))?;
            let value: serde_json::Value =
                decrypt_data(memorie["value"].as_str().unwrap().to_owned())?;

            if !map.contains_key(memorie["key"].as_str().unwrap()) {
                map.insert(memorie["key"].as_str().unwrap().to_owned(), value);
            }
        }
    }

    context.current = serde_json::Value::Object(map);
    context.metadata = metadata.clone();
    Ok(())
}
