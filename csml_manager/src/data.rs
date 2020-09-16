use crate::{Client, ContextJson};
use csml_interpreter::data::message::Message; //ApiInfo, Hold
use curl::easy::Easy;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[cfg(feature = "dynamo")]
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient as RusotoDynamoDbClient;

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
    #[cfg(feature = "dynamo")]
    Dynamodb(DynamoDbClient),
    None,
}

/**
 * Dynamodb runs in async by default and returns futures, that need to be awaited on.
 * The proper way to do it is by using tokio's runtime::block_on(). It is however quite costly
 * to setup, so let's just do it once in the base DynamoDbStruct here.
 */
#[cfg(feature = "dynamo")]
pub struct DynamoDbClient {
    pub client: RusotoDynamoDbClient,
    pub runtime: Option<tokio::runtime::Runtime>,
}
impl DynamoDbClient {
    pub fn new(region: Region) -> Self {
        Self {
            client: RusotoDynamoDbClient::new(region),
            runtime: None,
        }
    }
    pub fn get_runtime(&self) -> Result<tokio::runtime::Runtime, ManagerError> {
        match tokio::runtime::Runtime::new() {
            Ok(rt) => Ok(rt),
            Err(err) => Err(ManagerError::Manager(err.to_string())),
        }
    }
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

    #[cfg(any(feature = "dynamo"))]
    Rusoto(String),
    #[cfg(any(feature = "dynamo"))]
    SerdeDynamodb(serde_dynamodb::Error),
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

#[cfg(any(feature = "dynamo"))]
impl<E: std::error::Error + 'static> From<rusoto_core::RusotoError<E>> for ManagerError {
    fn from(e: rusoto_core::RusotoError<E>) -> Self {
        ManagerError::Rusoto(e.to_string())
    }
}

#[cfg(any(feature = "dynamo"))]
impl From<serde_dynamodb::Error> for ManagerError {
    fn from(e: serde_dynamodb::Error) -> Self {
        ManagerError::SerdeDynamodb(e)
    }
}
