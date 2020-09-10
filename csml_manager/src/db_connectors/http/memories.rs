use crate::db_connectors::http::state::format_state_data;
use http_db::models::CreateStateBody;
use crate::{
    Client, ConversationInfo, ManagerError, Memories,
};

use http_db::apis::client::APIClient;

pub fn format_memories(
    data: &mut ConversationInfo,
    memories: &[Memories],
    interaction_order: i32,
) -> Vec<CreateStateBody> {
    let vec = memories
        .iter()
        .fold(vec![], |mut vec: Vec<(&str, &serde_json::Value)>, var| {
            vec.push((&var.key, &var.value));
            vec
        });
    format_state_data(data, "remember", interaction_order, vec)
}

pub fn get_memories(
    client: &Client,
    db: &APIClient,
) -> Result<serde_json::Value, ManagerError> {
    let memories = db.memories_api()
        .get_memories(&client.bot_id, &client.user_id, &client.channel_id).unwrap();

    let map = memories.iter()
        .fold(serde_json::Map::new(), |mut map, mem| {
        if !map.contains_key(&mem.key) {
            map.insert(mem.key.clone(), mem.value.clone());
        }
        map
    });
    Ok(serde_json::json!(map))
}
