pub mod run;
pub mod conversations;
pub mod validate;
pub mod sns;
pub mod bot_versions;
pub mod bots;
pub mod clients;
pub mod memories;

use csml_engine::data::{RunRequest};
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
pub struct GetVersionsRequest {
    pub bot_id: String,
    pub limit: Option<i64>,
    pub pagination_key: Option<String>,
}
