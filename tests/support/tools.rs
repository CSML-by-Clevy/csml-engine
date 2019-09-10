use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use serde_json::{Value, json, map::Map};
use multimap::MultiMap;

use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[allow(dead_code)]
pub fn gen_context(
    past: MultiMap<String, MemoryType>,
    current: MultiMap<String, MemoryType>,
    metadata: MultiMap<String, MemoryType>,
    retries: i64,
    is_initial_step: bool,
) -> Context {
    Context {
        past,
        current,
        metadata,
        retries,
        is_initial_step,
        client: Client {
            bot_id: "none".to_owned(),
            channel_id: "none".to_owned(),
            user_id: "none".to_owned()
        },
        fn_endpoint: "none".to_owned()
    }
}

#[allow(dead_code)]
pub fn gen_event(event: &str) -> Event {
    Event {
        payload: PayLoad {
            content_type: "text".to_owned(),
            content: PayLoadContent {
                text: event.to_owned()
            }
        }
    }
}

#[allow(dead_code)]
pub fn gen_memory(key: &str, value: Value) -> MemoryType {
    MemoryType {
            created_at: "Today".to_owned(),
            step_id: None,
            flow_id: None,
            conversation_id: None,
            key: key.to_owned(),
            value,
    }
}

pub fn message_to_jsonvalue(result: MessageData) -> Value {
    let mut message: Map<String, Value> = Map::new();
    let mut vec = vec![];
    let mut memories = vec![];

    for msg in result.messages.iter() {
        vec.push(msg.to_owned().message_to_json());
    }

    if let Some(mem) = result.memories {
        for elem in mem.iter() {
            memories.push(elem.to_owned().memorie_to_jsvalue());
        }
    }

    message.insert("memories".to_owned(), Value::Array(memories));
    message.insert("messages".to_owned(), Value::Array(vec));
    message.insert("next_flow".to_owned(), match serde_json::to_value(result.next_flow) { Ok(val) => val, _ => json!(null)});
    message.insert("next_step".to_owned(), match serde_json::to_value(result.next_step) { Ok(val) => val, _ => json!(null)});

    Value::Object(message)
}

// metadata.insert(
//     "firstname".to_owned(),
//     gen_memory("firstname", Value::String("Alexis".to_owned()) )
// );
// metadata.insert(
//     "tutu".to_owned(),
//     gen_memory("tutu", Value::String("toto".to_owned()))
// );
