use crate::Client;

mod db;
#[cfg(feature = "http")]
mod db_interactions_http_db;
#[cfg(feature = "mongo")]
mod db_interactions_mongo;

pub use db::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Conversation {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String, // to_hex bson::oid::ObjectId
    pub client: Client,
    pub flow_id: String,             // to_hex
    pub step_id: String,             // to_hex
    pub metadata: serde_json::Value, // encrypted
    pub status: String,              //(OPEN, CLOSED, //Faild?
    pub last_interaction_at: String, // to_rfc3339_opts(SecondsFormat::Millis, true)
    pub updated_at: String,          // to_rfc3339_opts(SecondsFormat::Millis, true)
    pub created_at: String,          // to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Interaction {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String, // to_hex
    pub client: Client,
    pub success: bool,
    pub event: serde_json::Value, // encrypted
    pub updated_at: String,       // to_rfc3339_opts(SecondsFormat::Millis, true)
    pub created_at: String,       // to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DbMemories {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String, // to_hex
    pub client: Client,
    pub interaction_id: String,  // to_hex
    pub conversation_id: String, // to_hex
    pub flow_id: String,
    pub step_id: String,
    pub memory_order: i32,
    pub interaction_order: i32,
    pub key: String,
    pub value: String,              // encrypted
    pub expires_at: Option<String>, // to_rfc3339_opts(SecondsFormat::Millis, true)
    pub created_at: String,         // to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Messages {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String, // to_hex
    pub client: Client,
    pub interaction_id: String,  // to_hex
    pub conversation_id: String, // to_hex
    pub flow_id: String,
    pub step_id: String,
    pub message_order: i32,
    pub interaction_order: i32,
    pub direction: String,    // (SEND, RECEIVE)
    pub payload: String,      // encrypted
    pub content_type: String, // to_rfc3339_opts(SecondsFormat::Millis, true)
    pub created_at: String,   // to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Node {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub interaction_id: String,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub next_step: Option<String>,
    pub next_flow: Option<String>,
    pub created_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct State {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    #[serde(rename = "type")]
    pub _type: String,
    pub value: String, // encrypted
    pub expires_at: Option<String>,
    pub created_at: String,
}
// #[cfg(all(feature = "http", not(feature = "mongo")))]
// mod db_interactions_httpdb;

// #[cfg(all(feature = "mongo", not(feature = "http")))]
// #[cfg(all(feature = "http", not(feature = "mongo")))]
// use db_interactions_httpdb as db_interactions;
