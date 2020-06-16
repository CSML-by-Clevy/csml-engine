use crate::{
    db_interactions::db_interactions_dynamo::state::format_state_body, ConversationInfo, Memories,
};
use dynamodb::models::CreateStateBody;

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
    format_state_body(data, "remember", interaction_order, vec)
}
