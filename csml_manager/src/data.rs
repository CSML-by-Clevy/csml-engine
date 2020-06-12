use crate::{Client, ContextJson, CsmlBot};
use csmlinterpreter::data::message::Message; //ApiInfo, Hold
use curl::easy::Easy;
use serde_json::Value;

pub const DEBUG: &str = "DEBUG";
pub const DISABLE_SSL_VERIFY: &str = "DISABLE_SSL_VERIFY";

pub struct CsmlData {
    pub request_id: String,
    pub client: Client,
    pub callback_url: Option<String>,
    pub payload: Value,
    pub bot: CsmlBot,
    pub metadata: Value,
    pub sync: bool,
}

#[derive(Debug)]
pub enum Database {
    #[cfg(feature = "mongo")]
    Mongo(mongodb::Database),
    #[cfg(feature = "dynamodb")]
    Dynamodb(i32),
    None,
}

// #[derive(Serialize, Deserialize)]
// pub enum ContentType {
//     #[serde(rename = "url")]
//     Url,
//     #[serde(rename = "audio")]
//     Audio,
//     #[serde(rename = "video")]
//     Video,
//     #[serde(rename = "image")]
//     Image,
//     #[serde(rename = "payload")]
//     Payload,
//     #[serde(rename = "flow_trigger")]
//     FlowTrigger,
// }

pub struct ConversationInfo {
    // pub api_client: APIClient,
    pub request_id: String,
    pub curl: Option<Easy>,
    pub conversation_id: String,
    pub interaction_id: String,
    pub client: Client,
    pub context: ContextJson,
    pub metadata: Value,
    // pub flow_info: FlowInfo<'a>,
    // if switch the last_flow contains the info of the last flow | (flow_name, step_name) |
    pub last_flow: Option<(String, String)>,
    pub messages: Vec<Message>,
    pub db: Database,
}

#[derive(Debug)]
pub enum Next {
    Flow(String),
    Step(String),
    Recursive,
    Hold, //(i32)
    End,
    Error,
}

#[derive(Debug)]
pub enum ManagerError {
    Serde(serde_json::Error),
    Io(std::io::Error),
    Manager(String),
    Interpreter(String),
    Time(std::time::SystemTimeError),
    Openssl(openssl::error::ErrorStack),
    Base64(base64::DecodeError),

    #[cfg(any(feature = "mongo"))]
    BsonDecoder(bson::DecoderError),
    #[cfg(any(feature = "mongo"))]
    BsonEncoder(bson::EncoderError),
    #[cfg(any(feature = "mongo"))]
    MongoDB(mongodb::error::Error),
}

impl From<serde_json::Error> for ManagerError {
    fn from(e: serde_json::Error) -> Self {
        ManagerError::Serde(e)
    }
}

impl From<std::io::Error> for ManagerError {
    fn from(e: std::io::Error) -> Self {
        ManagerError::Io(e)
    }
}

impl From<std::time::SystemTimeError> for ManagerError {
    fn from(e: std::time::SystemTimeError) -> Self {
        ManagerError::Time(e)
    }
}

impl From<openssl::error::ErrorStack> for ManagerError {
    fn from(e: openssl::error::ErrorStack) -> Self {
        ManagerError::Openssl(e)
    }
}

impl From<base64::DecodeError> for ManagerError {
    fn from(e: base64::DecodeError) -> Self {
        ManagerError::Base64(e)
    }
}

#[cfg(any(feature = "mongo"))]
impl From<bson::EncoderError> for ManagerError {
    fn from(e: bson::EncoderError) -> Self {
        ManagerError::BsonEncoder(e)
    }
}

#[cfg(any(feature = "mongo"))]
impl From<bson::DecoderError> for ManagerError {
    fn from(e: bson::DecoderError) -> Self {
        ManagerError::BsonDecoder(e)
    }
}

#[cfg(any(feature = "mongo"))]
impl From<mongodb::error::Error> for ManagerError {
    fn from(e: mongodb::error::Error) -> Self {
        ManagerError::MongoDB(e)
    }
}
