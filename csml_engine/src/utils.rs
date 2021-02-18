use crate::{
    data::{ConversationInfo, Database, EngineError, DEBUG},
    db_connectors::state::delete_state_key,
    send::send_to_callback_url,
    CsmlBot, CsmlFlow,
};

use chrono::{prelude::Utc, SecondsFormat};
use csml_interpreter::{
    data::{ast::Flow, Client, Context, Event, Interval, Memory, Message},
    get_step,
    interpreter::json_to_literal,
};
use rand::seq::SliceRandom;
use serde_json::{json, map::Map, Value};
use std::collections::HashMap;
use std::env;

use md5::{Digest, Md5};

/**
 * Update current context memories in place.
 * This method is used to avoid saving memories in DB every time a `remember` is used
 * Instead, the memory is saved in bulk at the end of each step or interaction, but we still
 * must allow the user to use the `remembered` data immediately.
 */
pub fn update_current_context(data: &mut ConversationInfo, mem: &[Memory]) {
    for elem in mem.iter() {
        let lit = json_to_literal(&elem.value, Interval::default()).unwrap();

        data.context.current.insert(elem.key.to_owned(), lit);
    }
}

/**
 * Prepare a formatted "content" for the event object, based on the user's input.
 * This will trim extra data and only keep the main value.
 */
pub fn get_event_content(content_type: &str, metadata: &Value) -> Result<String, EngineError> {
    match content_type {
        file if ["file", "audio", "video", "image", "url"].contains(&file) => {
            if let Some(val) = metadata["url"].as_str() {
                Ok(val.to_string())
            } else {
                Err(EngineError::Interpreter(
                    "no url content in event".to_owned(),
                ))
            }
        }
        payload if payload == "payload" => {
            if let Some(val) = metadata["payload"].as_str() {
                Ok(val.to_string())
            } else {
                Err(EngineError::Interpreter(
                    "no payload content in event".to_owned(),
                ))
            }
        }
        text if text == "text" => {
            if let Some(val) = metadata["text"].as_str() {
                Ok(val.to_string())
            } else {
                Err(EngineError::Interpreter(
                    "no text content in event".to_owned(),
                ))
            }
        }
        flow_trigger if flow_trigger == "flow_trigger" => {
            if let Some(val) = metadata["flow_id"].as_str() {
                Ok(val.to_string())
            } else {
                Err(EngineError::Interpreter(
                    "no flow_id content in event".to_owned(),
                ))
            }
        }
        content_type => Err(EngineError::Interpreter(format!(
            "{} is not a valid content_type",
            content_type
        ))),
    }
}

/**
 * Format the incoming (JSON-formatted) event into an Event struct.
 */
pub fn format_event(json_event: serde_json::Value) -> Result<Event, EngineError> {
    let content_type = match json_event["payload"]["content_type"].as_str() {
        Some(content_type) => content_type.to_string(),
        None => {
            return Err(EngineError::Interpreter(
                "no content_type in event".to_owned(),
            ))
        }
    };
    let content = json_event["payload"]["content"].to_owned();

    let content_value = get_event_content(&content_type, &content)?;
    Ok(Event {
        content_type,
        content_value,
        content,
    })
}

/**
 * Send a message to the configured callback_url.
 * If not callback_url is configured, skip this action.
 */
pub fn send_msg_to_callback_url(
    data: &mut ConversationInfo,
    msg: Vec<Message>,
    interaction_order: i32,
    end: bool,
) {
    let messages = messages_formater(data, msg, interaction_order, end);

    match env::var(DEBUG) {
        Ok(ref var) if var == "true" => {
            println!("conversation_end => {}", messages["conversation_end"]);
        }
        _ => (),
    };

    match serde_json::to_string(&messages) {
        Ok(string) => send_to_callback_url(data, string.as_bytes()),
        Err(_err) => (),
    };
}

/**
 * Update ConversationInfo data with current information about the request.
 */
fn add_info_to_message(data: &ConversationInfo, mut msg: Message, interaction_order: i32) -> Value {
    let payload = msg.message_to_json();

    let mut map_msg: Map<String, Value> = Map::new();
    map_msg.insert("payload".to_owned(), payload);
    map_msg.insert("interaction_order".to_owned(), json!(interaction_order));
    map_msg.insert("conversation_id".to_owned(), json!(data.conversation_id));
    map_msg.insert("direction".to_owned(), json!("SEND"));

    Value::Object(map_msg)
}

/**
 * Prepare correctly formatted messages as requested in both:
 * - send action: when callback_url is set, messages are sent as they come to a defined endpoint
 * - return action: at the end of the interaction, all messages are returned as they were processed
 */
pub fn messages_formater(
    data: &mut ConversationInfo,
    vec_msg: Vec<Message>,
    interaction_order: i32,
    end: bool,
) -> Map<String, Value> {
    let msgs = vec_msg
        .into_iter()
        .map(|msg| add_info_to_message(data, msg, interaction_order))
        .collect();
    let mut map: Map<String, Value> = Map::new();

    map.insert("messages".to_owned(), Value::Array(msgs));
    map.insert("conversation_end".to_owned(), Value::Bool(end));

    map.insert("request_id".to_owned(), json!(data.request_id));
    map.insert(
        "received_at".to_owned(),
        json!(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
    );
    map.insert("interaction_id".to_owned(), json!(data.interaction_id));

    //tmp
    let mut map_client: Map<String, Value> = Map::new();
    map_client.insert("bot_id".to_owned(), json!(data.client.bot_id));
    map_client.insert("user_id".to_owned(), json!(data.client.user_id));
    map_client.insert("channel_id".to_owned(), json!(data.client.channel_id));
    map.insert("client".to_owned(), Value::Object(map_client));

    map
}

/**
 * Retrieve a flow in a given bot by an identifier:
 * - matching method is case insensitive
 * - as name is similar to a flow's alias, both flow.name and flow.id can be matched.
 */
pub fn get_flow_by_id<'a>(f_id: &str, flows: &'a [CsmlFlow]) -> Result<&'a CsmlFlow, EngineError> {
    let id = f_id.to_ascii_lowercase();
    // TODO: move to_lowercase at creation of vars
    match flows
        .iter()
        .find(|&val| val.id.to_ascii_lowercase() == id || val.name.to_ascii_lowercase() == id)
    {
        Some(ref f) => Ok(f),
        None => Err(EngineError::Interpreter(format!(
            "Flow '{}' does not exist",
            f_id
        ))),
    }
}

/**
 * Retrieve a bot's default flow.
 * The default flow must exist!
 */
pub fn get_default_flow<'a>(bot: &'a CsmlBot) -> Result<&'a CsmlFlow, EngineError> {
    match bot
        .flows
        .iter()
        .find(|&flow| flow.id == bot.default_flow || flow.name == bot.default_flow)
    {
        Some(flow) => Ok(flow),
        None => Err(EngineError::Interpreter(
            "The bot's default_flow does not exist".to_owned(),
        )),
    }
}

/**
 * Find a flow in a bot based on the user's input.
 * - flow_trigger events must will match a flow's id or name and reset the hold position
 * - other events will try to match a flow trigger
 */
pub fn search_flow<'a>(
    event: &Event,
    bot: &'a CsmlBot,
    client: &Client,
    db: &mut Database,
) -> Result<&'a CsmlFlow, EngineError> {
    match event {
        event if event.content_type == "flow_trigger" => {
            delete_state_key(&client, "hold", "position", db)?;
            get_flow_by_id(&event.content_value, &bot.flows)
        }
        event => {
            let mut random_flows = vec![];

            for flow in bot.flows.iter() {
                let contains_command = flow
                    .commands
                    .iter()
                    .any(|cmd| &cmd.as_str().to_lowercase() == &event.content_value.to_lowercase());

                if contains_command {
                    random_flows.push(flow)
                }
            }

            match random_flows.choose(&mut rand::thread_rng()) {
                Some(flow) => {
                    delete_state_key(&client, "hold", "position", db)?;
                    Ok(flow)
                }
                None => Err(EngineError::Interpreter(format!(
                    "Flow '{}' does not exist",
                    event.content_value
                ))),
            }
        }
    }
}

pub fn get_current_step_hash(context: &Context, bot: &CsmlBot) -> Result<String, EngineError> {
    let mut hash = Md5::new();

    let flow = &get_flow_by_id(&context.flow, &bot.flows)?.content;

    let ast = match &bot.bot_ast {
        Some(ast) => {
            let base64decoded = base64::decode(&ast).unwrap();
            let csml_bot: HashMap<String, Flow> = bincode::deserialize(&base64decoded[..]).unwrap();
            match csml_bot.get(&context.flow) {
                Some(flow) => flow.to_owned(),
                None => csml_bot
                    .get(&get_default_flow(&bot)?.name)
                    .unwrap()
                    .to_owned(),
            }
        }
        None => return Err(EngineError::Manager(format!("not valid ast"))),
    };

    let step = get_step(&context.step, &flow, &ast);
    hash.update(step.as_bytes());

    Ok(format!("{:x}", hash.finalize()))
}

pub fn clean_hold_and_restart(data: &mut ConversationInfo) -> Result<(), EngineError> {
    delete_state_key(&data.client, "hold", "position", &mut data.db)?;
    data.context.hold = None;
    return Ok(());
}
