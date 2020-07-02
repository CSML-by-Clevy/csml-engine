use csmlinterpreter::data::csml_bot::CsmlBot;
use csmlinterpreter::data::csml_flow::CsmlFlow;
use csmlinterpreter::data::event::Event;
use csmlinterpreter::data::message_data::MessageData;
use csmlinterpreter::data::ContextJson;
use csmlinterpreter::interpret;
use serde_json::{json, map::Map, Value};

use std::fs::File;
use std::io::prelude::*;

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[allow(dead_code)]
pub fn format_message(event: Event, context: ContextJson, filepath: &str) -> MessageData {
    let content = read_file(filepath.to_string()).unwrap();

    let flow = CsmlFlow::new("id", "flow", &content, Vec::default());
    let bot = CsmlBot::new("id", "bot", None, vec![flow], serde_json::json!({}), "flow");

    interpret(bot, context, event, None)
}

#[allow(dead_code)]
pub fn message_to_json_value(result: MessageData) -> Value {
    let mut message: Map<String, Value> = Map::new();
    let mut vec = vec![];
    let mut memories = vec![];

    for msg in result.messages.iter() {
        vec.push(msg.to_owned().message_to_json());
    }

    if let Some(mem) = result.memories {
        for elem in mem.iter() {
            let mut map = Map::new();
            map.insert("key".to_owned(), json!(elem.key.to_owned()));
            map.insert("value".to_owned(), elem.value.to_owned());
            //TODO: UPDATE
            // map.insert(elem.key.to_owned(), elem.value.to_owned());
            memories.push(json!(map));
        }
    }

    message.insert("memories".to_owned(), Value::Array(memories));
    message.insert("messages".to_owned(), Value::Array(vec));

    Value::Object(message)
}
