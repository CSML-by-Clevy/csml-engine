use crate::{Client, ContextJson};
use csml_interpreter::data::message::Message; //ApiInfo, Hold
use curl::easy::Easy;
use serde_json::Value;
use serde::{Deserialize, Serialize};

pub const DEBUG: &str = "DEBUG";
pub const DISABLE_SSL_VERIFY: &str = "DISABLE_SSL_VERIFY";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsmlRequest {
  pub request_id: String,
  pub client: Client,
  pub callback_url: Option<String>,
  pub payload: serde_json::Value,
  pub metadata: serde_json::Value,
}

pub enum Database {
    #[cfg(feature = "mongo")]
    Mongo(mongodb::Database),
    #[cfg(feature = "http")]
    Httpdb(http_db::apis::client::APIClient),
    None,
}

pub struct ConversationInfo {
    pub request_id: String,
    pub curl: Option<Easy>,
    pub conversation_id: String,
    pub interaction_id: String,
    pub client: Client,
    pub context: ContextJson,
    pub metadata: Value,
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

    #[cfg(any(feature = "http"))]
    Reqwest(reqwest::Error),
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

#[cfg(any(feature = "http"))]
impl From<reqwest::Error> for ManagerError {
    fn from(e: reqwest::Error) -> Self {
        ManagerError::Reqwest(e)
    }
}

#[cfg(any(feature = "http"))]
impl From<http_db::apis::Error> for ManagerError {
    fn from(e: http_db::apis::Error) -> Self {
        match e {
            http_db::apis::Error::Reqwest(reqwest) => ManagerError::Reqwest(reqwest),
            http_db::apis::Error::Serde(serde) => ManagerError::Serde(serde),
            http_db::apis::Error::Io(io) => ManagerError::Io(io),
            http_db::apis::Error::Interpreter(string) => ManagerError::Interpreter(string),
        }
    }
}
