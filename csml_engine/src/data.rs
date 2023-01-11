#[cfg(feature = "pooled")]
use diesel::r2d2::{ConnectionManager, PooledConnection, R2D2Connection};
use crate::{
    db_connectors,
    encrypt::{decrypt_data, encrypt_data},
    Client, Context,
};
use csml_interpreter::data::{CsmlBot, CsmlFlow, Message, Module, MultiBot};
use serde::de::StdError;
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
    #[serde(alias = "fn_endpoint")]
    pub apps_endpoint: Option<String>,
    pub multibot: Option<Vec<MultiBot>>,
    pub event: CsmlRequest,
}

impl RunRequest {
    pub fn get_bot_opt(&self) -> Result<BotOpt, EngineError> {
        match self.clone() {
            // Bot
            RunRequest {
                bot: Some(mut csml_bot),
                multibot,
                ..
            } => {
                csml_bot.multibot = multibot;

                Ok(BotOpt::CsmlBot(csml_bot))
            }

            // version id
            RunRequest {
                version_id: Some(version_id),
                bot_id: Some(bot_id),
                apps_endpoint,
                multibot,
                ..
            } => Ok(BotOpt::Id {
                version_id,
                bot_id,
                apps_endpoint,
                multibot,
            }),

            // get bot by id will search for the last version id
            RunRequest {
                bot_id: Some(bot_id),
                apps_endpoint,
                multibot,
                ..
            } => Ok(BotOpt::BotId {
                bot_id,
                apps_endpoint,
                multibot,
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
        #[serde(alias = "fn_endpoint")]
        apps_endpoint: Option<String>,
        multibot: Option<Vec<MultiBot>>,
    },
    #[serde(rename = "bot_id")]
    BotId {
        bot_id: String,
        #[serde(alias = "fn_endpoint")]
        apps_endpoint: Option<String>,
        multibot: Option<Vec<MultiBot>>,
    },
}

impl BotOpt {
    pub fn search_bot(&self, db: &mut Database) -> Result<CsmlBot, EngineError> {
        match self {
            BotOpt::CsmlBot(csml_bot) => Ok(csml_bot.to_owned()),
            BotOpt::BotId {
                bot_id,
                apps_endpoint,
                multibot,
            } => {
                let bot_version = db_connectors::bot::get_last_bot_version(bot_id, db)?;

                match bot_version {
                    Some(mut bot_version) => {
                        bot_version.bot.apps_endpoint = apps_endpoint.to_owned();
                        bot_version.bot.multibot = multibot.to_owned();
                        Ok(bot_version.bot)
                    }
                    None => Err(EngineError::Manager(format!(
                        "bot ({}) not found in db",
                        bot_id
                    ))),
                }
            }
            BotOpt::Id {
                version_id,
                bot_id,
                apps_endpoint,
                multibot,
            } => {
                let bot_version = db_connectors::bot::get_by_version_id(version_id, bot_id, db)?;

                match bot_version {
                    Some(mut bot_version) => {
                        bot_version.bot.apps_endpoint = apps_endpoint.to_owned();
                        bot_version.bot.multibot = multibot.to_owned();
                        Ok(bot_version.bot)
                    }
                    None => Err(EngineError::Manager(format!(
                        "bot version ({}) not found in db",
                        version_id
                    ))),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializeCsmlBot {
    pub id: String,
    pub name: String,
    pub flows: Vec<CsmlFlow>,
    pub native_components: Option<String>,
    // serde_json::Map<String, serde_json::Value>
    pub custom_components: Option<String>,
    // serde_json::Value
    pub default_flow: String,
    pub no_interruption_delay: Option<i32>,
    pub env: Option<String>,
    pub modules: Option<Vec<Module>>,
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
    pub native_components: Option<String>,
    // serde_json::Map<String, serde_json::Value>
    pub custom_components: Option<String>,
    // serde_json::Value
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
            modules: None,
        }
    }
}

pub fn to_serializable_bot(bot: &CsmlBot) -> SerializeCsmlBot {
    SerializeCsmlBot {
        id: bot.id.to_owned(),
        name: bot.name.to_owned(),
        flows: bot.flows.to_owned(),
        native_components: {
            bot.native_components.to_owned().map(|value| serde_json::Value::Object(value).to_string())
        },
        custom_components: {
            bot.custom_components.to_owned().map(|value| value.to_string())
        },
        default_flow: bot.default_flow.to_owned(),
        no_interruption_delay: bot.no_interruption_delay,
        env: match &bot.env {
            Some(value) => encrypt_data(value).ok(),
            None => None,
        },
        modules: bot.modules.to_owned(),
    }
}

impl SerializeCsmlBot {
    pub fn to_bot(&self) -> CsmlBot {
        CsmlBot {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            apps_endpoint: None,
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
            modules: self.modules.to_owned(),
            multibot: None,
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
        custom_components: csml_bot.custom_components.to_owned().map(|value| value.to_string()),
        default_flow: csml_bot.default_flow.to_owned(),
        no_interruption_delay: csml_bot.no_interruption_delay,
        env: match &csml_bot.env {
            Some(value) => encrypt_data(value).ok(),
            None => None,
        },
    }
}

impl DynamoBot {
    pub fn to_bot(&self, flows: Vec<CsmlFlow>, modules: Vec<Module>) -> CsmlBot {
        CsmlBot {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            apps_endpoint: None,
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
            modules: Some(modules),
            multibot: None,
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
    pub step_limit: Option<usize>,
    pub ttl_duration: Option<serde_json::Value>,
    pub low_data_mode: Option<serde_json::Value>,
}

#[cfg(feature = "pooled")]
pub enum Connections<'a, E: R2D2Connection + 'static> {
    Direct(E),
    Reference(&'a mut E),
    Managed(&'a mut PooledConnection<ConnectionManager<E>>),
}

#[cfg(feature = "pooled")]
impl<'a, E: R2D2Connection> AsMut<E> for Connections<'a, E> {
    fn as_mut(&mut self) -> &mut E {
        match self {
            Connections::Direct(e) => e,
            Connections::Reference(e) => e,
            #[cfg(feature = "pooled")]
            Connections::Managed(e) => e,
        }
    }
}

#[cfg(not(feature = "pooled"))]
pub enum Connections<'a, E> {
    Direct(E),
    Reference(&'a mut E),
}

#[cfg(not(feature = "pooled"))]
impl<'a, E> AsMut<E> for Connections<'a, E> {
    fn as_mut(&mut self) -> &mut E {
        match self {
            Connections::Direct(e) => e,
            Connections::Reference(e) => e
        }
    }
}

pub enum Database<'a> {
    #[cfg(feature = "mongo")]
    Mongo(MongoDbClient),
    #[cfg(feature = "dynamo")]
    Dynamodb(DynamoDbClient),
    #[cfg(feature = "postgresql")]
    Postgresql(PostgresqlClient<'a>),
    #[cfg(feature = "sqlite")]
    SqLite(SqliteClient<'a>),
    None,
}

#[cfg(feature = "sqlite")]
pub struct SqliteClient<'a> {
    pub client: Connections<'a, diesel::prelude::SqliteConnection>,
}

#[cfg(feature = "sqlite")]
impl SqliteClient<'static> {
    pub fn new(client: diesel::prelude::SqliteConnection) -> Self {
        Self { client: Connections::Direct(client)}
    }
}

#[cfg(feature = "postgresql")]
pub struct PostgresqlClient<'a> {
    pub client: Connections<'a, diesel::prelude::PgConnection>,
}

#[cfg(feature = "postgresql")]
impl PostgresqlClient<'static> {
    pub fn new(client: diesel::prelude::PgConnection) -> Self {
        Self { client: Connections::Direct(client)}
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

pub struct ConversationInfo<'a> {
    pub request_id: String,
    pub conversation_id: String,
    pub callback_url: Option<String>,
    pub client: Client,
    pub context: Context,
    pub metadata: Value,
    pub messages: Vec<Message>,
    pub ttl: Option<chrono::Duration>,
    pub low_data: bool,
    pub db: Database<'a>,
}

#[derive(Debug)]
pub enum Next {
    Flow(String),
    Step(String),
    Hold,
    //(i32)
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
    DateTimeError(String),
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

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    SqlErrorCode(String),
    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    SqlMigrationsError(String),
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

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl From<diesel::result::Error> for EngineError {
    fn from(e: diesel::result::Error) -> Self {
        EngineError::SqlErrorCode(e.to_string())
    }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl From<Box<dyn StdError + Send + Sync>> for EngineError {
    fn from(e: Box<dyn StdError + Send + Sync>) -> Self {
        EngineError::SqlErrorCode(e.to_string())
    }
}

#[cfg(any(feature = "postgresql", feature = "sqlite"))]
impl From<diesel_migrations::MigrationError> for EngineError {
    fn from(e: diesel_migrations::MigrationError) -> Self {
        EngineError::SqlMigrationsError(e.to_string())
    }
}
