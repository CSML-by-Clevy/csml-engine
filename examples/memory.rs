use csmlinterpreter::{
    data::{ContextJson, Event, MessageData},
    interpret,
};
use serde_json::{json, map::Map, Value};

use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn format_message(result: &MessageData) -> Value {
    let mut message: Map<String, Value> = Map::new();
    let mut vec = vec![];
    let mut memories = vec![];

    for msg in result.messages.iter() {
        vec.push(msg.to_owned().message_to_json());
    }

    if let Some(ref mem) = result.memories {
        for elem in mem.iter() {
            let mut map = Map::new();
            map.insert(elem.key.to_owned(), elem.value.to_owned());
            memories.push(json!(map));
        }
    }

    message.insert("memories".to_owned(), Value::Array(memories));
    message.insert("messages".to_owned(), Value::Array(vec));
    message.insert(
        "next_flow".to_owned(),
        match serde_json::to_value(result.next_flow.to_owned()) {
            Ok(val) => val,
            _ => json!(null),
        },
    );
    message.insert(
        "next_step".to_owned(),
        match serde_json::to_value(result.next_step.to_owned()) {
            Ok(val) => val,
            _ => json!(null),
        },
    );
    Value::Object(message)
}

fn interpret_flow(flow: &str) {
    let event = Event::text("hello");
    let mut metadata = Map::new();
    let mut memories = Map::new();

    metadata.insert("firstname".to_owned(), json!("Toto"));
    metadata.insert("email".to_owned(), json!("toto@clevy.com"));

    memories.insert(
        "tmp".to_owned(), 
        serde_json::json!({
            "_content": {
                "cards":[
                    {
                        "_content":{
                            "buttons":[{"_content":{"accepts":["b1"],"icon":"info","payload":"b1","theme":"primary","title":"b1","toto":{"test":"plop"}},"_content_type":"button"}],
                            "title":"c1"
                        },
                        "_content_type":"card"
                    }
                ]
            },
            "_content_type":"carousel"
        })
    );

    let mut context = ContextJson {
        current: serde_json::json!(memories),
        metadata: json!(metadata),
        api_info: None,
        hold: None,
    };
    let mut step = "start".to_owned();
    let mut memory = Map::new();

    while step != "end" {
        let messages = interpret(flow, &step, context.clone(), &event, None);
        dbg!(format_message(&messages));

        if let Some(mem) = messages.memories {
            for res in mem.iter() {
                memory.insert(res.key.to_owned(), res.value.to_owned());
            }
        }

        context.current = serde_json::json!(memory);

        match &messages.next_step {
            Some(new_step) => step = new_step.to_owned(),
            None => step = "end".to_owned(),
        }
    }
}

fn main() {
    let flow = read_file("CSML/examples/memory.csml".to_owned()).unwrap();

    interpret_flow(&flow);
}

// memoriers.insert(
//     "tmp".to_owned(),
//     serde_json::json!({
//         "_content": {
//             "cards":[
//                 {
//                     "_content":{
//                         "buttons":[{"_content":{"accepts":["b1"],"icon":"info","payload":"b1","theme":"primary","title":"b1","toto":{"test":"plop"}},"_content_type":"button"}],
//                         "title":"c1"
//                     },
//                     "_content_type":"card"
//                 }
//             ]
//         },
//         "_content_type":"carousel"
//     })
// );
