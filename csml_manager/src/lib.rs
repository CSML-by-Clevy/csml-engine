pub mod data;
pub use csmlinterpreter::data::{
    csml_result::CsmlResult, error_info::ErrorInfo, warnings::Warnings, Client,
};

mod db_interactions;
#[cfg(any(feature = "mongo"))]
mod encrypt;
mod init;
mod interpreter_actions;
mod send;
mod tools;

use data::*;
use db_interactions::{conversation::*, init_db, messages::*, state::*, Conversation};
use init::*;
use interpreter_actions::interpret_step;
use tools::*;

use csmlinterpreter::data::{csml_bot::CsmlBot, csml_flow::CsmlFlow, ContextJson, Hold, Memories};
use md5::{Digest, Md5};
use std::{env, time::SystemTime, collections::HashMap};

pub fn start_conversation(
    json_event: serde_json::Value,
    csmldata: CsmlData,
) -> Result<serde_json::Map<String, serde_json::Value>, ManagerError> {
    let now = SystemTime::now();

    let event = format_event(json_event.clone())?;
    let mut data = init_conversation_info(
        get_default_flow(&csmldata.bot)?.name.to_owned(),
        &event,
        &csmldata,
    )?;
    // save event in db as message RECEIVE
    let msgs = vec![json_event["payload"].to_owned()];
    add_messages_bulk(&mut data, msgs, 0, "RECEIVE")?;

    let flow = get_flow_by_id(&data.context.flow, &csmldata.bot.flows)?;
    check_for_hold(&mut data, flow)?;

    let res = interpret_step(&mut data, event.to_owned(), &csmldata);

    if let Ok(var) = env::var(DEBUG) {
        if var == "true" {
            let el = now.elapsed()?;
            println!("Total time Manager - {}.{}", el.as_secs(), el.as_millis());
        }
    }
    res
}

pub fn get_open_conversation(client: &Client) -> Result<Option<Conversation>, ManagerError> {
    let db = init_db()?;

    get_latest_open(client, &db)
}

pub fn get_steps_from_flow(bot: CsmlBot) -> HashMap<String, Vec<String>>{
    csmlinterpreter::get_steps_from_flow(bot)
}

pub fn validate_bot(bot: CsmlBot) -> CsmlResult {
    csmlinterpreter::validate_bot(bot)
}

pub fn user_close_all_conversations(client: Client) -> Result<(), ManagerError> {
    let db = init_db()?;

    delete_state_key(&client, "hold", "position", &db)?;
    close_all_conversations(&client, &db)
}

// reset memory if flow hash is different or see if there are some save tmp memories
fn check_for_hold(data: &mut ConversationInfo, flow: &CsmlFlow) -> Result<(), ManagerError> {
    match get_state_key(&data.client, "hold", "position", &data.db) {
        Ok(Some(string)) => {
            let hold = serde_json::to_value(string)?;
            let mut hash = Md5::new();

            hash.input(flow.content.as_bytes());
            let new_hash = format!("{:x}", hash.result());

            if new_hash != hold["hash"] {
                data.context.step = "start".to_owned();
                delete_state_key(&data.client, "hold", "position", &data.db)?;
                data.context.hold = None;
                return Ok(());
            }
            data.context.hold = Some(Hold {
                index: hold["index"].as_u64().ok_or(ManagerError::Interpreter(
                    "hold index bad format".to_owned(),
                ))? as usize,
                step_vars: hold["step_vars"].clone(),
            });
            delete_state_key(&data.client, "hold", "position", &data.db)?;
        }
        Ok(None) => (),
        Err(_) => (),
    };
    Ok(())
}
