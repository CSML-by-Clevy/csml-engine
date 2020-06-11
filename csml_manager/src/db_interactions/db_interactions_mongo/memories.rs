use crate::{
    encrypt::{decrypt_data, encrypt_data},
    Client, ContextJson, ConversationInfo, ManagerError, Memories,
    db_interactions::DbMemories,
    db_interactions::db_interactions_mongo::get_db,
};
use bson::{doc, Bson};

pub fn format_memories(
    data: &mut ConversationInfo,
    memories: &[Memories],
) -> Result<Vec<bson::Document>, ManagerError> {
    let client = bson::to_bson(&data.client)?;

    let vec = memories
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
                "interaction_order": 0, //tmp
                "key": &var.key,
                "value": value, // encrypted
                "expires_at": Bson::Null,
                "created_at": time
            });
            Ok(vec)
        });

    vec
}

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &[Memories],
) -> Result<(), ManagerError> {
    if memories.len() == 0 {
        return Ok(());
    }
    let mem = format_memories(data, memories)?;
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
            let memorie: DbMemories = bson::from_bson(bson::Bson::Document(doc))?;
            let value: serde_json::Value = decrypt_data(memorie.value)?;

            if !map.contains_key(&memorie.key) {
                map.insert(memorie.key, value);
            }
        }
    }

    context.current = serde_json::Value::Object(map);
    context.metadata = metadata.clone();
    Ok(())
}
