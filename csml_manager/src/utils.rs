use crate::{
    data::{ConversationInfo, Database, ManagerError, DEBUG},
    db_connectors::state::delete_state_key,
    send::send_to_callback_url,
    CsmlBot, CsmlFlow,
};

use chrono::{prelude::Utc, SecondsFormat};
use csml_interpreter::data::{Client, Event, Memories, Message};
use serde_json::{json, map::Map, Value};
use std::env;

/**
 * Update current context memories in place.
 * This method is used to avoid saving memories in DB every time a `remember` is used
 * Instead, the memory is saved in bulk at the end of each step or interaction, but we still
 * must allow the user to use the `remembered` data immediately.
 */
pub fn update_current_context(data: &mut ConversationInfo, mem: &[Memories]) {
    for elem in mem.iter() {
        if let Value::Object(ref mut obj) = data.context.current {
            obj.insert(elem.key.to_owned(), elem.value.clone());
        }
    }
}

/**
 * Prepare a formatted "content" for the event object, based on the user's input.
 * This will trim extra data and only keep the main value.
 */
pub fn get_event_content(content_type: &str, metadata: &Value) -> Result<String, ManagerError> {
    match content_type {
        file if ["file", "audio", "video", "image", "url"].contains(&file) => {
            if let Some(val) = metadata["url"].as_str() {
                Ok(val.to_string())
            } else {
                Err(ManagerError::Interpreter(
                    "no url content in event".to_owned(),
                ))
            }
        }
        payload if payload == "payload" => {
            if let Some(val) = metadata["payload"].as_str() {
                Ok(val.to_string())
            } else {
                Err(ManagerError::Interpreter(
                    "no payload content in event".to_owned(),
                ))
            }
        }
        text if text == "text" => {
            if let Some(val) = metadata["text"].as_str() {
                Ok(val.to_string())
            } else {
                Err(ManagerError::Interpreter(
                    "no text content in event".to_owned(),
                ))
            }
        }
        flow_trigger if flow_trigger == "flow_trigger" => {
            if let Some(val) = metadata["flow_id"].as_str() {
                Ok(val.to_string())
            } else {
                Err(ManagerError::Interpreter(
                    "no flow_id content in event".to_owned(),
                ))
            }
        }
        content_type => Err(ManagerError::Interpreter(format!(
            "{} is not a valid content_type",
            content_type
        ))),
    }
}

/**
 * Format the incoming (JSON-formatted) event into an Event struct.
 */
pub fn format_event(json_event: serde_json::Value) -> Result<Event, ManagerError> {
    let content_type = match json_event["payload"]["content_type"].as_str() {
        Some(content_type) => content_type.to_string(),
        None => {
            return Err(ManagerError::Interpreter(
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
fn add_info_to_message(
    data: &ConversationInfo,
    mut msg: Message,
    interaction_order: i32,
) -> Value {
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
pub fn get_flow_by_id<'a>(f_id: &str, flows: &'a [CsmlFlow]) -> Result<&'a CsmlFlow, ManagerError> {
    // TODO: move to_lowercase at creation of vars
    match flows.iter().find(
        |&val| val.id == f_id || val.name.to_ascii_lowercase() == f_id.to_ascii_lowercase(), // || val.commands.contains(&f_id.to_ascii_lowercase())
    ) {
        Some(ref f) => Ok(f),
        None => Err(ManagerError::Interpreter(format!(
            "Flow '{}' does not exist",
            f_id
        ))),
    }
}

/**
 * Retrieve a bot's default flow.
 * The default flow must exist!
 */
pub fn get_default_flow<'a>(bot: &'a CsmlBot) -> Result<&'a CsmlFlow, ManagerError> {
    match bot
        .flows
        .iter()
        .find(|&flow| flow.id == bot.default_flow || flow.name == bot.default_flow)
    {
        Some(flow) => Ok(flow),
        None => Err(ManagerError::Interpreter(
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
    db: &Database,
) -> Result<&'a CsmlFlow, ManagerError> {
    match event {
        event if event.content_type == "flow_trigger" => {
            delete_state_key(&client, "hold", "position", db)?;
            get_flow_by_id(&event.content_value, &bot.flows)
        }
        event => {
            for flow in bot.flows.iter() {
                for command in flow.commands.iter() {
                    if &command.to_lowercase() == &event.content_value.to_lowercase() {
                        delete_state_key(&client, "hold", "position", db)?;
                        return Ok(flow);
                    }
                }
            }
            Err(ManagerError::Interpreter(format!(
                "Flow '{}' does not exist",
                event.content_value
            )))
        }
    }
}
