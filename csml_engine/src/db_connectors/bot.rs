#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{Client, ConversationInfo, Database, DbConversation, EngineError, CsmlBot};
use csml_interpreter::data::ast::Flow;

use std::collections::HashMap;

pub fn save_bot_state(
    bot_id: String,
    bot: String,
    db: &mut Database,
) -> Result<String, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::save_bot_state(
            bot_id, bot, db
        );
    }

    // #[cfg(feature = "dynamo")]
    // if is_dynamodb() {
    //     let db = dynamodb_connector::get_db(db)?;
    //     return dynamodb_connector::conversations::create_conversation(
    //         flow_id, step_id, client, metadata, db,
    //     );
    // }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_last_bot_version(
    bot_id: &str,
    db: &mut Database,
) -> Result<Option< CsmlBot >, EngineError> { //HashMap<String, Flow>
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::get_last_bot_version(&bot_id, db);
    }

    // #[cfg(feature = "dynamo")]
    // if is_dynamodb() {
    //     let db = dynamodb_connector::get_db(db)?;
    //     return dynamodb_connector::conversations::create_conversation(
    //         flow_id, step_id, client, metadata, db,
    //     );
    // }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_by_id(
    id: &str,
    db: &mut Database,
) -> Result<Option< CsmlBot >, EngineError> { //HashMap<String, Flow>
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::get_bot_by_id(&id, db);
    }

    // #[cfg(feature = "dynamo")]
    // if is_dynamodb() {
    //     let db = dynamodb_connector::get_db(db)?;
    //     return dynamodb_connector::conversations::create_conversation(
    //         flow_id, step_id, client, metadata, db,
    //     );
    // }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_bot_list(
    bot_id: &str,
    db: &mut Database,
) -> Result<Vec< serde_json::Value >, EngineError> { //HashMap<String, Flow>
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::bot::get_bot_list(&bot_id, db);
    }

    // #[cfg(feature = "dynamo")]
    // if is_dynamodb() {
    //     let db = dynamodb_connector::get_db(db)?;
    //     return dynamodb_connector::conversations::create_conversation(
    //         flow_id, step_id, client, metadata, db,
    //     );
    // }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}