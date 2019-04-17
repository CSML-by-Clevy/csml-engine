use serde::{Deserialize, Serialize};
use multimap::MultiMap;

// Node module
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsContext {
    pub past: Option< Vec<serde_json::Value> >,
    pub current: Option< Vec<serde_json::Value> >,
    pub metadata: Option< Vec<serde_json::Value> >,
}

// -----------------------------------------------

//  Object({
//         "created_at": String("2019-04-17T08:13:42.000Z"),
//         "flow_name": String("form"),
//         "step_name": Null,
//         "key": String("firstname"),
//         "type": Null,
//         "value": String("Alexis")
//  })

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum ValueType {
//     STR(String),
//     BOOL(bool),
//     I32(i32),
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryType {
    // pub conversation_id: String,
    pub created_at: String,
    pub flow_name: Option<String>,
    pub step_name: Option<String>,
    pub key: String,
    pub r#type: Option<String>,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memory {
    pub past: MultiMap<String, MemoryType>,
    pub current: MultiMap<String, MemoryType>,
    pub metadata: MultiMap<String, MemoryType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub text: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayLoad {
    pub content_type: String,
    pub content: Content
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub channel_id: String,
    pub channel_type: String,
    pub user_id: String,
    pub timestamp: i64,
    pub payload: PayLoad
}