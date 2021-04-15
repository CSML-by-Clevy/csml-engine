#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
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
        let serializable_bot = crate::data::to_serializable_bot(&csml_bot);
        let bot = serde_json::json!(serializable_bot).to_string();

        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::create_bot_version(bot_id, bot, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        let dynamo_bot = crate::data::to_dynamo_bot(&csml_bot);

        let flows = serde_json::json!(&csml_bot.flows);
        let bot = serde_json::json!(dynamo_bot).to_string();

        let version_id = dynamodb_connector::bot::create_bot_version(
            bot_id.clone(),
            bot,
            flows.to_string(),
            db,
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
    pagination_key: Option<String>,
    db: &mut Database,
) -> Result<serde_json::Value, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::get_bot_versions(&bot_id, limit, pagination_key, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;

        let pagination_key = match pagination_key {
            Some(key) => {
                let base64decoded = match base64::decode(&key) {
                    Ok(base64decoded) => base64decoded,
                    Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
                };
                match serde_json::from_slice(&base64decoded) {
                    Ok(key) => Some(key),
                    Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
                }
            }
            None => None,
        };

        return dynamodb_connector::bot::get_bot_versions(&bot_id, limit, pagination_key, db);
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


pub fn delete_all_bot_data(bot_id: &str, db: &mut Database) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        delete_bot_versions(bot_id, db)?;

        let db = mongodb_connector::get_db(db)?;

        mongodb_connector::bot::delete_all_bot_data(bot_id, "memory", db)?;
        mongodb_connector::bot::delete_all_bot_data(bot_id, "message", db)?;
        mongodb_connector::bot::delete_all_bot_data(bot_id, "interaction", db)?;
        mongodb_connector::bot::delete_all_bot_data(bot_id, "conversation", db)?;
        mongodb_connector::bot::delete_all_bot_data(bot_id, "state", db)?;
        mongodb_connector::bot::delete_all_bot_data(bot_id, "path", db)?;

        return Ok(());
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        delete_bot_versions(bot_id, db)?;

        let db = dynamodb_connector::get_db(db)?;

        dynamodb_connector::bot::delete_all_bot_data(bot_id, "memory", db)?;
        dynamodb_connector::bot::delete_all_bot_data(bot_id, "message", db)?;
        dynamodb_connector::bot::delete_all_bot_data(bot_id, "interaction", db)?;
        dynamodb_connector::bot::delete_all_bot_data(bot_id, "conversation", db)?;
        dynamodb_connector::bot::delete_all_bot_data(bot_id, "state", db)?;
        return Ok(());
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}




