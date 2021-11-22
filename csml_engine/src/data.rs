use crate::{
    db_connectors,
    encrypt::{decrypt_data, encrypt_data},
    Client, Context,
};
use csml_interpreter::data::{CsmlBot, CsmlFlow, Message};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const DEBUG: &str = "DEBUG";
pub const DISABLE_SSL_VERIFY: &str = "DISABLE_SSL_VERIFY";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlowTrigger {
    pub flow_id: String,
    pub step_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunRequest {
    pub bot: Option<CsmlBot>,
    pub bot_id: Option<String>,
    pub version_id: Option<String>,
    pub fn_endpoint: Option<String>,
    pub event: CsmlRequest,
}

impl RunRequest {
    pub fn get_bot_opt(&self) -> Result<BotOpt, EngineError> {
        match self.clone() {
            RunRequest {
                bot: Some(csml_bot),
                ..
            } => Ok(BotOpt::CsmlBot(csml_bot)),
            RunRequest {
                version_id: Some(version_id),
                bot_id: Some(bot_id),
                fn_endpoint,
                ..
            } => Ok(BotOpt::Id {
                version_id,
                bot_id,
                fn_endpoint,
            }),
            RunRequest {
                bot_id: Some(bot_id),
                fn_endpoint,
                ..
            } => Ok(BotOpt::BotId {
                bot_id,
                fn_endpoint,
            }),
            _ => Err(EngineError::Format("Invalid bot_opt format".to_owned())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BotOpt {
    #[serde(rename = "bot")]
    CsmlBot(CsmlBot),
    #[serde(rename = "version_id")]
    Id {
        version_id: String,
        bot_id: String,
        fn_endpoint: Option<String>,
    },
    #[serde(rename = "bot_id")]
    BotId {
        bot_id: String,
        fn_endpoint: Option<String>,
    },
}

impl BotOpt {
    pub fn search_bot(&self, db: &mut Database) -> CsmlBot {
        match self {
            BotOpt::CsmlBot(csml_bot) => csml_bot.to_owned(),
            BotOpt::BotId {
                bot_id,
                fn_endpoint,
            } => {
                let mut bot_version = db_connectors::bot::get_last_bot_version(&bot_id, db)
                    .unwrap()
                    .unwrap();
                bot_version.bot.fn_endpoint = fn_endpoint.to_owned();
                bot_version.bot
            }
            BotOpt::Id {
                version_id,
                bot_id,
                fn_endpoint,
            } => {
                let mut bot_version =
                    db_connectors::bot::get_by_version_id(&version_id, &bot_id, db)
                        .unwrap()
                        .unwrap();
                bot_version.bot.fn_endpoint = fn_endpoint.to_owned();
                bot_version.bot
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializeCsmlBot {
    pub id: String,
    pub name: String,
    pub flows: Vec<CsmlFlow>,
    pub native_components: Option<String>, // serde_json::Map<String, serde_json::Value>
    pub custom_components: Option<String>, // serde_json::Value
    pub default_flow: String,
    pub no_interruption_delay: Option<i32>,
    pub env: Option<String>,
}

/**
 * Before CSML v1.5, the Bot struct was encoded with bincode. This does not
 * allow to easily change the contents of a bot, and would not allow to add
 * the bot env feature.
 * We need to keep this for backwards compatibility until CSML v2.
 * TO BE REMOVED in CSML v2
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsmlBotBincode {
    pub id: String,
    pub name: String,
    pub flows: Vec<CsmlFlow>,
    pub native_components: Option<String>, // serde_json::Map<String, serde_json::Value>
    pub custom_components: Option<String>, // serde_json::Value
    pub default_flow: String,
}

impl CsmlBotBincode {
    pub fn to_bot(self) -> SerializeCsmlBot {
        SerializeCsmlBot {
            id: self.id,
            name: self.name,
            flows: self.flows,
            native_components: self.native_components,
            custom_components: self.custom_components,
            default_flow: self.default_flow,
            no_interruption_delay: None,
            env: None,
        }
    }
}

pub fn to_serializable_bot(bot: &CsmlBot) -> SerializeCsmlBot {
    SerializeCsmlBot {
        id: bot.id.to_owned(),
        name: bot.name.to_owned(),
        flows: bot.flows.to_owned(),
        native_components: {
            match bot.native_components.to_owned() {
                Some(value) => Some(serde_json::Value::Object(value).to_string()),
                None => None,
            }
        },
        custom_components: {
            match bot.custom_components.to_owned() {
                Some(value) => Some(value.to_string()),
                None => None,
            }
        },
        default_flow: bot.default_flow.to_owned(),
        no_interruption_delay: bot.no_interruption_delay,
        env: match &bot.env {
            Some(value) => encrypt_data(value).ok(),
            None => None,
        },
    }
}

impl SerializeCsmlBot {
    pub fn to_bot(&self) -> CsmlBot {
        CsmlBot {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            fn_endpoint: None,
            flows: self.flows.to_owned(),
            native_components: {
                match self.native_components.to_owned() {
                    Some(value) => match serde_json::from_str(&value) {
                        Ok(serde_json::Value::Object(map)) => Some(map),
                        _ => unreachable!(),
                    },
                    None => None,
                }
            },
            custom_components: {
                match self.custom_components.to_owned() {
                    Some(value) => match serde_json::from_str(&value) {
                        Ok(value) => Some(value),
                        Err(_e) => unreachable!(),
                    },
                    None => None,
                }
            },
            default_flow: self.default_flow.to_owned(),
            bot_ast: None,
            no_interruption_delay: self.no_interruption_delay,
            env: match self.env.to_owned() {
                Some(value) => decrypt_data(value).ok(),
                None => None,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoBot {
    pub id: String,
    pub name: String,
    pub custom_components: Option<String>,
    pub default_flow: String,
    pub no_interruption_delay: Option<i32>,
    pub env: Option<String>,
}

/**
 * Before CSML v1.5, the Bot struct was encoded with bincode. This does not
 * allow to easily change the contents of a bot, and would not allow to add
 * the bot env feature.
 * We need to keep this for backwards compatibility until CSML v2.
 * TO BE REMOVED in CSML v2
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoBotBincode {
    pub id: String,
    pub name: String,
    pub custom_components: Option<String>,
    pub default_flow: String,
}

impl DynamoBotBincode {
    pub fn to_bot(self) -> DynamoBot {
        DynamoBot {
            id: self.id,
            name: self.name,
            custom_components: self.custom_components,
            default_flow: self.default_flow,
            no_interruption_delay: None,
            env: None,
        }
    }
}

pub fn to_dynamo_bot(csml_bot: &CsmlBot) -> DynamoBot {
    DynamoBot {
        id: csml_bot.id.to_owned(),
        name: csml_bot.name.to_owned(),
        custom_components: match csml_bot.custom_components.to_owned() {
            Some(value) => Some(value.to_string()),
            None => None,
        },
        default_flow: csml_bot.default_flow.to_owned(),
        no_interruption_delay: csml_bot.no_interruption_delay,
        env: match &csml_bot.env {
            Some(value) => encrypt_data(value).ok(),
            None => None,
        },
    }
}

impl DynamoBot {
    pub fn to_bot(&self, flows: Vec<CsmlFlow>) -> CsmlBot {
        CsmlBot {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            fn_endpoint: None,
            flows,
            native_components: None,
            custom_components: {
                match self.custom_components.to_owned() {
                    Some(value) => match serde_json::from_str(&value) {
                        Ok(value) => Some(value),
                        Err(_e) => unreachable!(),
                    },
                    None => None,
                }
            },
            default_flow: self.default_flow.to_owned(),
            bot_ast: None,
            no_interruption_delay: self.no_interruption_delay,
            env: match self.env.to_owned() {
                Some(value) => decrypt_data(value).ok(),
                None => None,
            },
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
    Mongo(MongoDbClient),
    #[cfg(feature = "dynamo")]
    Dynamodb(DynamoDbClient),
    #[cfg(feature = "postgresql")]
    Postgresql(PostgresqlClient),
    None,
}


#[cfg(feature = "postgresql")]
pub struct PostgresqlClient {
    pub client: diesel::prelude::PgConnection,
}

#[cfg(feature = "postgresql")]
impl PostgresqlClient {
    pub fn new(client: diesel::prelude::PgConnection) -> Self {
        Self { client }
    }
}

#[cfg(feature = "mongo")]
pub struct MongoDbClient {
    pub client: mongodb::sync::Database,
}

#[cfg(feature = "mongo")]
impl MongoDbClient {
    pub fn new(client: mongodb::sync::Database) -> Self {
        Self { client }
    }
}
/**
 * Dynamodb runs in async by default and returns futures, that need to be awaited on.
 * The proper way to do it is by using tokio's runtime::block_on(). It is however quite costly
 * to setup, so let's just do it once in the base DynamoDbStruct here.
 */
#[cfg(feature = "dynamo")]
pub struct DynamoDbClient {
    pub client: rusoto_dynamodb::DynamoDbClient,
    pub s3_client: rusoto_s3::S3Client,
    pub runtime: tokio::runtime::Runtime,
}

#[cfg(feature = "dynamo")]
impl DynamoDbClient {
    pub fn new(dynamo_region: rusoto_core::Region, s3_region: rusoto_core::Region) -> Self {
        Self {
            client: rusoto_dynamodb::DynamoDbClient::new(dynamo_region),
            s3_client: rusoto_s3::S3Client::new(s3_region),
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

pub struct ConversationInfo {
    pub request_id: String,
    pub conversation_id: String,
    pub callback_url: Option<String>,
    pub client: Client,
    pub context: Context,
    pub metadata: Value,
    pub messages: Vec<Message>,
    pub ttl: Option<chrono::Duration>,
    pub low_data: bool,
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
    Utf8(std::str::Utf8Error),
    Manager(String),
    Format(String),
    Interpreter(String),
    Parring(String),
    Time(std::time::SystemTimeError),
    Openssl(openssl::error::ErrorStack),
    Base64(base64::DecodeError),

    #[cfg(any(feature = "mongo"))]
    BsonDecoder(bson::de::Error),
    #[cfg(any(feature = "mongo"))]
    BsonEncoder(bson::ser::Error),
    #[cfg(any(feature = "mongo"))]
    MongoDB(mongodb::error::Error),

    #[cfg(any(feature = "dynamo"))]
    Rusoto(String),
    #[cfg(any(feature = "dynamo"))]
    SerdeDynamodb(serde_dynamodb::Error),
    #[cfg(any(feature = "dynamo"))]
    S3ErrorCode(u16),

    #[cfg(any(feature = "postgresql"))]
    PsqlErrorCode(String),
    #[cfg(any(feature = "postgresql"))]
    PsqlMigrationsError(String),
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

impl From<std::str::Utf8Error> for EngineError {
    fn from(e: std::str::Utf8Error) -> Self {
        EngineError::Utf8(e)
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
impl From<bson::de::Error> for EngineError {
    fn from(e: bson::de::Error) -> Self {
        EngineError::BsonDecoder(e)
    }
}

#[cfg(any(feature = "mongo"))]
impl From<bson::ser::Error> for EngineError {
    fn from(e: bson::ser::Error) -> Self {
        EngineError::BsonEncoder(e)
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

#[cfg(any(feature = "postgresql"))]
impl From<diesel::result::Error> for EngineError {
    fn from(e: diesel::result::Error) -> Self {
        EngineError::PsqlErrorCode(e.to_string())
    }
}

#[cfg(any(feature = "postgresql"))]
impl From<diesel_migrations::RunMigrationsError> for EngineError {
    fn from(e: diesel_migrations::RunMigrationsError) -> Self {
        EngineError::PsqlMigrationsError(e.to_string())
    }
}
