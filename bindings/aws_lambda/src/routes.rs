pub mod run;
pub mod conversations;
pub mod validate;
pub mod sns;
pub mod create_bot_version;
pub mod get_bot_by_version_id;
pub mod get_bot_versions;
pub mod get_last_bot_version;

use csml_engine::data::CsmlRequest;
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct RunRequest {
    bot: CsmlBot,
    event: CsmlRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetByIdRequest {
    id: String,
    bot_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVersionsRequest {
    bot_id: String,
    limit: Option<i64>,
    last_key: Option<String>,
}