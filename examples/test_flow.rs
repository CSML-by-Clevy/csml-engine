use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::{ast::*, literal::Literal};
use csmlinterpreter::{interpret, parse_file};
use std::collections::HashMap;
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
    // None;
    let event = Some(Event {
        payload: "Trending".to_owned(), // payload: PayLoad {
                                        //     content_type: "text".to_owned(),
                                        //     content: PayLoadContent {
                                        //         text: "Trending".to_owned()
                                        //     }
                                        // }
    });
    // Some(Event{
    //     literal: Literal::string("42".to_owned())
    // });
    let mut metadata = HashMap::new();

    metadata.insert(
        "firstname".to_owned(),
        Literal::string("Alexis".to_owned(), Interval { column: 0, line: 0 }),
    );

    metadata.insert(
        "mavar".to_owned(),
        Literal::int(10, Interval { column: 0, line: 0 }),
    );

    let mut obj = HashMap::new();
    obj.insert(
        "var1".to_owned(),
        Literal::int(1, Interval { column: 0, line: 0 }),
    );
    obj.insert(
        "var2".to_owned(),
        Literal::int(10, Interval { column: 0, line: 0 }),
    );
    metadata.insert(
        "obj".to_owned(),
        Literal::object(obj, Interval { column: 0, line: 0 }),
    );

    let mut context = Context {
        past: HashMap::new(),
        current: HashMap::new(),
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
        flow,
        step_name,
        &mut context,
        &event,
        None,
        None
    )));
}

fn main() {
    let text = read_file("CSML/test.csml".to_owned()).unwrap();

    let flow = match parse_file(&text) {
        Ok(flow) => flow,
        Err(e) => panic!("{:?}", e),
    };

    interpret_flow(&flow, "start");
}

// ((1 + 3) > 6) && var
// if (true == (7 && (7 - (3+(2*1)) == 2)))) {
//     goto toto
// }
// "{{(1 + 3)}} {{var}}"
// ((1 + 3) > 6) + var
