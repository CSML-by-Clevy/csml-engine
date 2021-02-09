#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "dynamo")]
use csml_interpreter::data::csml_bot::DynamoBot;
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{BotVersion, CsmlBot, Database, EngineError};

pub fn create_bot_version(
    bot_id: String,
    csml_bot: CsmlBot,
    db: &mut Database,
) -> Result<String, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let serializable_bot = csml_bot.to_serializable_bot();
        let bot = base64::encode(bincode::serialize(&serializable_bot).unwrap());

        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::create_bot_version(bot_id, bot, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;

        let dynamo_bot = DynamoBot {
            id: csml_bot.id.to_owned(),
            name: csml_bot.name.to_owned(),
            custom_components: match csml_bot.custom_components.to_owned() {
                Some(value) => Some(value.to_string()),
                None => None,
            },
            default_flow: csml_bot.default_flow.to_owned(),
        };

        let flows = serde_json::json!(&csml_bot.flows);
        let bot = base64::encode(bincode::serialize(&dynamo_bot).unwrap());
        let version_id = dynamodb_connector::bot::create_bot_version(
            bot_id.clone(),
            bot,
            flows.to_string(),
            db
        )?;

        return Ok(version_id);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_last_bot_version(
    bot_id: &str,
    db: &mut Database,
) -> Result<Option<BotVersion>, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::get_last_bot_version(&bot_id, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::bot::get_last_bot_version(&bot_id, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_by_version_id(
    version_id: &str,
    _bot_id: &str,
    db: &mut Database,
) -> Result<Option<BotVersion>, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::get_bot_by_version_id(&version_id, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::bot::get_bot_by_version_id(&version_id, &_bot_id, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_bot_versions(
    bot_id: &str,
    limit: Option<i64>,
    last_key: Option<String>,
    db: &mut Database,
) -> Result<serde_json::Value, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::get_bot_versions(&bot_id, limit, last_key, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::bot::get_bot_versions(&bot_id, limit, last_key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn delete_bot_version(
    _bot_id: &str,
    version_id: &str,
    db: &mut Database,
) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::delete_bot_version(version_id, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::bot::delete_bot_version(_bot_id, version_id, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn delete_bot_versions(bot_id: &str, db: &mut Database) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::delete_bot_versions(bot_id, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::bot::delete_bot_versions(bot_id, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
