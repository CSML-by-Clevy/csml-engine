/**
 * This module defines the interactions between the CSML Manager and the underlying
 * database engines.
 *
 * There are several engines to choose from (see module features). To use one
 * of the database options, the ENGINE_DB_TYPE env var must be set
 * to one of the accepted values:
 *
 * - `mongodb`: requires a MongoDB-compatible database and additional variables:
 *   - MONGODB_HOST
 *   - MONGODB_PORT
 *   - MONGODB_DATABASE
 *   - MONGODB_USERNAME
 *   - MONGODB_PASSWORD
 *
 * - `dynamodb`: requires a DynamoDB-compatible database (on AWS, or dynamodb-local
 * for dev purposes). A S3-compatible storage is also needed for storing bots in the engine.
 * The following env vars are required (alternatively if deploying on AWS, prefer using IAM roles)
 *   - AWS_REGION
 *   - AWS_ACCESS_KEY_ID
 *   - AWS_SECRET_ACCESS_KEY
 *   - AWS_DYNAMODB_TABLE
 *   - AWS_DYNAMODB_ENDPOINT optional, defaults to the dynamodb endpoint for the given region.
 *   - AWS_S3_BUCKET
 *   - AWS_S3_ENDPOINT optional, defaults to the S3 endpoint for the given region
 * Both AWS_REGION AND AWS_DYNAMODB_ENDPOINT must be set to use a custom dynamodb-compatible DB.
 *
 * If the ENGINE_DB_TYPE env var is not set, mongodb is used by default.
 *
 * To add a new DB type, please use one of the existing templates implementations.
 * Each method of each module must be fully reimplemented in order to extend the "generic"
 * implementation at the root of db_connectors directory.
 */
use crate::data::{Database, EngineError};
use crate::error_messages::ERROR_DB_SETUP;
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};

#[cfg(feature = "dynamo")]
use self::dynamodb as dynamodb_connector;
#[cfg(feature = "mongo")]
use self::mongodb as mongodb_connector;
#[cfg(feature = "postgresql")]
use self::postgresql as postgresql_connector;

pub mod bot;
pub mod conversations;
pub mod memories;
pub mod messages;
pub mod state;

pub mod user;
pub mod clean_db;
pub mod utils;

pub mod db_test;

use crate::Client;

#[cfg(feature = "dynamo")]
mod dynamodb;
#[cfg(feature = "mongo")]
mod mongodb;
#[cfg(feature = "postgresql")]
mod postgresql;


#[derive(Serialize, Deserialize, Debug)]
pub struct DbConversation {
    pub id: String,
    pub client: Client,
    pub flow_id: String,
    pub step_id: String,
    // pub metadata: serde_json::Value,
    pub status: String,
    pub last_interaction_at: String,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbMemory {
    pub id: String,
    pub client: Client,
    pub interaction_id: String,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub memory_order: i32,
    pub interaction_order: i32,
    pub key: String,
    pub value: serde_json::Value,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbMessage {
    pub id: String,
    pub client: Client,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub message_order: i32,
    pub interaction_order: i32,
    pub direction: String,
    pub payload: serde_json::Value,
    pub content_type: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbState {
    pub id: String,
    pub client: Client,
    #[serde(rename = "type")]
    pub _type: String,
    pub value: serde_json::Value,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbBot {
    pub id: String,
    pub bot_id: String,
    pub bot: String,
    pub engine_version: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotVersion {
    pub bot: CsmlBot,
    pub version_id: String,
    pub engine_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotVersionCreated {
    pub version_id: String,
    pub engine_version: String,
}

impl BotVersion {
    pub fn flatten(&self) -> serde_json::Value {
        let mut value = serde_json::json!(self.bot);

        value["version_id"] = serde_json::json!(self.version_id);
        value["engine_version"] = serde_json::json!(self.engine_version);

        value
    }
}

#[cfg(feature = "mongo")]
pub fn is_mongodb() -> bool {
    // If the env var is not set at all, use mongodb by default
    match std::env::var("ENGINE_DB_TYPE") {
        Ok(val) => val == "mongodb".to_owned(),
        Err(_) => true,
    }
}

#[cfg(feature = "dynamo")]
pub fn is_dynamodb() -> bool {
    match std::env::var("ENGINE_DB_TYPE") {
        Ok(val) => val == "dynamodb".to_owned(),
        Err(_) => false,
    }
}

#[cfg(feature = "postgresql")]
pub fn is_postgresql() -> bool {
    match std::env::var("ENGINE_DB_TYPE") {
        Ok(val) => val == "postgresql".to_owned(),
        Err(_) => false,
    }
}

pub fn init_db() -> Result<Database, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::init();
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        return dynamodb_connector::init();
    }

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        return postgresql_connector::init();
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn make_migrations() -> Result<(), EngineError> {

    #[cfg(feature = "postgresql")]
    if is_postgresql() {
        return self::postgresql::make_migrations();
    }

    Ok(())
}
