pub mod bot_versions;
pub mod bots;
pub mod clients;
pub mod conversations;
pub mod memories;
pub mod messages;
pub mod migrations;
pub mod run;
pub mod sns;
pub mod state;
pub mod validate;

pub mod clean_data;

use csml_engine::data::RunRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BotIdVersionIdPath {
    pub version_id: String,
    pub bot_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotIdPath {
    pub bot_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryKeyPath {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryBody {
    pub key: String,
    pub value: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetVersionsRequest {
    pub bot_id: String,
    pub limit: Option<i64>,
    pub pagination_key: Option<String>,
}
