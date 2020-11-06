pub mod run;
pub mod conversations;
pub mod validate;
pub mod sns;

use csml_engine::data::CsmlRequest;
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct RunRequest {
    bot: CsmlBot,
    event: CsmlRequest,
}


