use crate::{db_connectors, Client, Context};
use csml_interpreter::data::{csml_bot::CsmlBot, Message};
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const DEBUG: &str = "DEBUG";
pub const DISABLE_SSL_VERIFY: &str = "DISABLE_SSL_VERIFY";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BotOpt {
    #[serde(rename = "csml_bot")]
    CsmlBot(CsmlBot),
    #[serde(rename = "id")]
    Id{id: String, bot_id: String, fn_endpoint: Option<String>},
    #[serde(rename = "bot_id")]
    BotId{bot_id: String, fn_endpoint: Option<String>}
}

impl BotOpt {
    pub fn search_bot(&self, db: &mut Database) -> CsmlBot {
        match self {
            BotOpt::CsmlBot(csml_bot) => csml_bot.to_owned(),
            BotOpt::BotId{bot_id, fn_endpoint} => {
                let mut bot = db_connectors::bot::get_last_bot_version(&bot_id, db).unwrap().unwrap();
                bot.fn_endpoint = fn_endpoint.to_owned();
                bot
            },
            BotOpt::Id{id, bot_id, fn_endpoint} => {
                let mut bot = db_connectors::bot::get_by_id(&id, &bot_id, db).unwrap().unwrap();
                bot.fn_endpoint = fn_endpoint.to_owned();
                bot
            }
        }
    }
}

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
    pub client: rusoto_dynamodb::DynamoDbClient,
    pub runtime: tokio::runtime::Runtime,
}

#[cfg(feature = "dynamo")]
impl DynamoDbClient {
    pub fn new(region: rusoto_core::Region) -> Self {
        Self {
            client: rusoto_dynamodb::DynamoDbClient::new(region),
            runtime: { tokio::runtime::Runtime::new().unwrap() },
        }
    }
}

pub struct ConversationInfo {
    pub request_id: String,
    pub curl: Option<Easy>,
    pub conversation_id: String,
    pub interaction_id: String,
    pub client: Client,
    pub context: Context,
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
pub enum EngineError {
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

    #[cfg(any(feature = "dynamo"))]
    Rusoto(String),
    #[cfg(any(feature = "dynamo"))]
    SerdeDynamodb(serde_dynamodb::Error),
}

impl From<serde_json::Error> for EngineError {
    fn from(e: serde_json::Error) -> Self {
        EngineError::Serde(e)
    }
}

impl From<std::io::Error> for EngineError {
    fn from(e: std::io::Error) -> Self {
        EngineError::Io(e)
    }
}

impl From<std::time::SystemTimeError> for EngineError {
    fn from(e: std::time::SystemTimeError) -> Self {
        EngineError::Time(e)
    }
}

impl From<openssl::error::ErrorStack> for EngineError {
    fn from(e: openssl::error::ErrorStack) -> Self {
        EngineError::Openssl(e)
    }
}

impl From<base64::DecodeError> for EngineError {
    fn from(e: base64::DecodeError) -> Self {
        EngineError::Base64(e)
    }
}

#[cfg(any(feature = "mongo"))]
impl From<bson::EncoderError> for EngineError {
    fn from(e: bson::EncoderError) -> Self {
        EngineError::BsonEncoder(e)
    }
}

#[cfg(any(feature = "mongo"))]
impl From<bson::DecoderError> for EngineError {
    fn from(e: bson::DecoderError) -> Self {
        EngineError::BsonDecoder(e)
    }
}

#[cfg(any(feature = "mongo"))]
impl From<mongodb::error::Error> for EngineError {
    fn from(e: mongodb::error::Error) -> Self {
        EngineError::MongoDB(e)
    }
}

#[cfg(any(feature = "dynamo"))]
impl<E: std::error::Error + 'static> From<rusoto_core::RusotoError<E>> for EngineError {
    fn from(e: rusoto_core::RusotoError<E>) -> Self {
        EngineError::Rusoto(e.to_string())
    }
}

#[cfg(any(feature = "dynamo"))]
impl From<serde_dynamodb::Error> for EngineError {
    fn from(e: serde_dynamodb::Error) -> Self {
        EngineError::SerdeDynamodb(e)
    }
}
