use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::ast::*;
use csmlinterpreter::{interpret, parse_file};
use multimap::MultiMap;
use serde_json::{json, map::Map, Value};

use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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
    message.insert(
        "next_flow".to_owned(),
        match serde_json::to_value(result.next_flow) {
            Ok(val) => val,
            _ => json!(null),
        },
    );
    message.insert(
        "next_step".to_owned(),
        match serde_json::to_value(result.next_step) {
            Ok(val) => val,
            _ => json!(null),
        },
    );

    Value::Object(message)
}

fn interpret_flow(flow: &Flow, step_name: &str) {
    let event = None;
    // Some(Event{
    //     payload: PayLoad {
    //         content_type: "text".to_owned(),
    //         content: PayLoadContent {
    //             text: "4".to_owned()
    //         }
    //     }
    // });
    // Some(Event{
    //     literal: Literal::string("42".to_owned())
    // });

    // let v: Value = serde_json::from_str(data).unwrap();

    let mut metadata = MultiMap::new();

    metadata.insert(
        "firstname".to_owned(),
        MemoryType {
            created_at: "Today".to_owned(),
            step_id: None,
            flow_id: None,
            conversation_id: None,
            key: "firstname".to_owned(),
            value: serde_json::Value::String("Alexis".to_owned()),
        },
    );

    metadata.insert(
        "mavar".to_owned(),
        MemoryType {
            created_at: "Today".to_owned(),
            step_id: None,
            flow_id: None,
            conversation_id: None,
            key: "mavar".to_owned(),
            value: json!(10),
        },
    );

    let memory = Context {
        past: MultiMap::new(),
        current: MultiMap::new(),
        metadata: metadata,
        retries: 42,
        is_initial_step: false,
        client: Client {
            bot_id: "1".to_owned(),
            channel_id: "2".to_owned(),
            user_id: "3".to_owned(),
        },
        fn_endpoint: "toto".to_owned(),
    };

    dbg!(message_to_jsonvalue(interpret(
        flow, step_name, &memory, &event
    )));
}

// use std::{env, io::Read};
// use curl::easy::{Easy, List};

fn main() {
    let text = read_file("CSML/test.csml".to_owned()).unwrap();

    let flow = match parse_file(&text) {
        Ok(flow) => flow,
        Err(e) => panic!("{:?}", e),
    };

    interpret_flow(&flow, "start");

    // println!("flow -> {:?}", flow);
    // println!("n-------------------------------------njson -> {:?}", serde_json::to_string(&flow).unwrap() );
}

// ((1 + 3) > 6) && var

// if (true == (7 && (7 - (3+(2*1)) == 2)))) {
//     goto toto
// }

// List(1, 2) => [1, 2]

// "{{(1 + 3)}} {{var}}"

// ((1 + 3) > 6) + var
