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
// ({
//  "bot_name": String("e_lisa"),
//  "channel_id": String("alexisbot"),
//  "channel_type": String("workplace"),
//  "conversation_id": String("db0c7ac3-9272-4165-94b7-e256bb35c308"),
//  "created_at": String("2019-04-16T14:28:54.000Z"),
//  "flow_name": String("form"),
//  "id": String("e018ec5f-4913-4bcb-8d36-3f717802acd7"),
//  "interaction_id": String("26b889e6-be3a-43a4-9edd-5fb6245e9c8e"),
//  "key": String("points"),
//  "step_name": String("STEP_25"),
//  "type": String("mem"),
//  "user_id": String("100033646789991"),
//  "value": Number(1)}
// )

// Object({
//         "conversation_id": String("24ac7583-af66-4f5a-9232-f029b6a3c8ed"),
//         "created_at": String("2019-04-16T16:10:37.000Z"),
//         "key": String("firstname"),
//         "value": String("Alexis")}
//     )


// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum ValueType {
//     STR(String),
//     BOOL(bool),
//     I32(i32),
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryType {
    // pub bot_name: String,
    // pub channel_id: String,
    // pub channel_type: String,
    pub conversation_id : String,
    pub created_at : String,
    // pub flow_name : String,
    // pub id : String,
    // pub interaction_id : String,
    pub key : String,
    // pub step_name : String,
    // pub r#type : String,
    // pub user_id : String,
    pub value : String
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