pub mod data;
pub use csml_interpreter::data::{
    csml_result::CsmlResult, error_info::ErrorInfo, warnings::Warnings, Client,
};
use serde_json::json;

mod db_connectors;
mod error_messages;

mod encrypt;
mod init;
mod interpreter_actions;
mod send;
mod utils;

use data::*;
use db_connectors::{bot, conversations::*, init_db, messages::*, state::*, DbConversation};
use init::*;
use interpreter_actions::interpret_step;
use utils::*;

use csml_interpreter::{
    data::{csml_bot::CsmlBot, csml_flow::CsmlFlow, Context, Hold, Memory},
    load_components,
};
use md5::{Digest, Md5};
use std::{collections::HashMap, env, time::SystemTime};

/**
 * Initiate a CSML chat request.
 * Takes 2 arguments: the request being made and the CSML bot.
 * This method assumes that the bot is already validated in advance. A best practice is
 * to pre-validate the bot and store it in a valid state.
 *
 * The request must be made by a given client. Its unicity (used as a key for identifying
 * who made each new request and if they relate to an already-open conversation) is based
 * on a combination of 3 parameters that are assumed to be unique in their own context:
 * - bot_id: differentiate bots handled by the same CSML engine instance
 * - channel_id: a given bot may be used on different channels (messenger, slack...)
 * - user_id: differentiate users on the same communication channel
 */
pub fn start_conversation(
    request: CsmlRequest,
    run_opt: BotOpt,
) -> Result<serde_json::Map<String, serde_json::Value>, EngineError> {
    let now = SystemTime::now();

    let formatted_event = format_event(json!(request))?;
    let mut db = init_db()?;
    let mut bot = run_opt.search_bot(&mut db);

    // load native components into the bot
    bot.native_components = match load_components() {
        Ok(components) => Some(components),
        Err(err) => return Err(EngineError::Interpreter(err.format_error())),
    };

    let mut data = init_conversation_info(
        get_default_flow(&bot)?.name.to_owned(),
        &formatted_event,
        &request,
        &bot,
        db,
    )?;

    // save event in db as message RECEIVE
    let msgs = vec![request.payload.to_owned()];
    add_messages_bulk(&mut data, msgs, 0, "RECEIVE")?;

    let flow = get_flow_by_id(&data.context.flow, &bot.flows)?;
    check_for_hold(&mut data, flow)?;

    let res = interpret_step(&mut data, formatted_event.to_owned(), &bot);

    if let Ok(var) = env::var(DEBUG) {
        if var == "true" {
            let el = now.elapsed()?;
            println!("Total time Manager - {}.{}", el.as_secs(), el.as_millis());
        }
    }
    res
}

/**
 * Return the latest conversation that is still open for a given user
 * (there should not be more than one), or None if there isn't any.
 */
pub fn get_open_conversation(client: &Client) -> Result<Option<DbConversation>, EngineError> {
    let mut db = init_db()?;

    get_latest_open(client, &mut db)
}

/**
 * create bot state
 * 
 * { "statusCode": i32, "body": version_id }
 */
pub fn create_bot_version(csml_bot: CsmlBot) -> Result<String, EngineError> {
    let mut db = init_db()?;

    let bot_id = csml_bot.id.clone();

    match validate_bot(csml_bot.clone()) {
        CsmlResult {
            errors: Some(errors),
            ..
        } => Err(EngineError::Interpreter(format!("{:?}", errors))),
        CsmlResult { .. } => bot::create_bot_version(bot_id, csml_bot, &mut db),
    }
}

/**
 * get by bot_id
 * {"statusCode": i32, "body": bot}
 *
 * bot: {
 *   id: String,
 *   name: String,
 *   fn_endpoint: Option<String>,
 *   flows: Vec<CsmlFlow>,
 *   custom_components: Option<>,
 *   default_flow: String,
 * }
 */
pub fn get_last_bot_version(bot_id: &str) -> Result<Option<CsmlBot>, EngineError>{
    let mut db = init_db()?;

    bot::get_last_bot_version(bot_id, &mut db)
}

/**
 * get bot by id
 * {"statusCode": i32, "body": bot}
 *
 * bot: {
 *   id: String,
 *   name: String,
 *   fn_endpoint: Option<String>,
 *   flows: Vec<CsmlFlow>,
 *   custom_components: Option<>,
 *   default_flow: String,
 * }
 */
pub fn get_bot_by_version_id(id: &str, bot_id: &str) -> Result<Option<CsmlBot>, EngineError> {
    let mut db = init_db()?;

    bot::get_by_id(id, bot_id, &mut db)
}

/**
 * List the last 20 versions of the bot if no limit is set
 *
 * "body": {"bots": Vec<bot_info>, "last_key": String}
 * 
 * bot_info = {
 *  "version_id": String,
 *  "bot": bot,
 *  "engine_version": String
 *  "created_at": String
 * }
 * 
 * bot: {
 *  "id": String,
 *  "name": String,
 *  "custom_components": Option<String>,
 *  "default_flow": String
 * } 
 */
pub fn get_bot_versions(
    bot_id: &str,
    limit: Option<i64>,
    last_key: Option<String>,
) -> Result<serde_json::Value, EngineError> {
    let mut db = init_db()?;

    bot::get_bot_versions(bot_id, limit, last_key, &mut db)
    // {
    //     Ok(value) => Ok(serde_json::json!({
    //         "statusCode": 200,
    //         "body": value
    //     })
    //     .to_string()),
    //     Err(err) => {
    //         let error = format!("{:?}", err);
    //         Ok(serde_json::json!({
    //             "statusCode": 404,
    //             "body": error
    //         })
    //         .to_string())
    //     }
    // }
}

/**
 * List all the steps in every flow of a given CSML bot
 */
pub fn get_steps_from_flow(bot: CsmlBot) -> HashMap<String, Vec<String>> {
    csml_interpreter::get_steps_from_flow(bot)
}

/**
 * Simple static CSML bot linter.
 * Does not check for possible runtime errors, only for build-time errors
 * (missing steps or flows, syntax errors, etc.)
 */
pub fn validate_bot(bot: CsmlBot) -> CsmlResult {
    csml_interpreter::validate_bot(bot)
}

/**
 * Close any open conversation a given client may currently have.
 * We also need to both clean the hold/local memory state to make sure
 * that outdated variables or hold positions are not loaded into the next open conversation.
 */
pub fn user_close_all_conversations(client: Client) -> Result<(), EngineError> {
    let mut db = init_db()?;

    delete_state_key(&client, "hold", "position", &mut db)?;
    close_all_conversations(&client, &mut db)
}

/**
 * Verify if the user is currently on hold in a given conversation.
 *
 * If a hold is found, make sure that the flow has not been updated since last conversation.
 * If that's the case, we can not be sure that the hold is in the same position,
 * so we need to clear the hold's position and restart the conversation.
 *
 * If the hold is valid, we also need to load the local step memory
 * (context.hold.step_vars) into the conversation context.
 */
fn check_for_hold(data: &mut ConversationInfo, flow: &CsmlFlow) -> Result<(), EngineError> {
    match get_state_key(&data.client, "hold", "position", &mut data.db) {
        // user is currently on hold
        Ok(Some(string)) => {
            let hold = serde_json::to_value(string)?;
            let mut hash = Md5::new();

            hash.update(flow.content.as_bytes());
            let new_hash = format!("{:x}", hash.finalize());

            // cleanup the current hold and restart flow
            if new_hash != hold["hash"] {
                data.context.step = "start".to_owned();
                delete_state_key(&data.client, "hold", "position", &mut data.db)?;
                data.context.hold = None;
                return Ok(());
            }

            // all good, let's load the position and local variables
            data.context.hold = Some(Hold {
                index: hold["index"]
                    .as_u64()
                    .ok_or(EngineError::Interpreter("hold index bad format".to_owned()))?
                    as usize,
                step_vars: hold["step_vars"].clone(),
            });
            delete_state_key(&data.client, "hold", "position", &mut data.db)?;
        }
        // user is not on hold
        Ok(None) => (),
        Err(_) => (),
    };
    Ok(())
}
