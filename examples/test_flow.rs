use csmlinterpreter::{
    data::{Client, ContextJson, Event, MessageData},
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

pub fn message_to_jsonvalue(result: MessageData) -> Value {
    let mut message: Map<String, Value> = Map::new();
    let mut vec = vec![];
    let mut memories = vec![];

    for msg in result.messages.iter() {
        vec.push(msg.to_owned().message_to_json());
    }

    if let Some(mem) = result.memories {
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

fn interpret_flow(flow: &str, step_name: &str) {
    // let event = None;
    let event = Some(Event {
        content_type: "payload".to_owned(),
        content: "plop".to_owned(),
        metadata: json!(null),
    });
    let mut metadata = Map::new();

    metadata.insert("firstname".to_owned(), json!("Alexis"));

    metadata.insert("mavar".to_owned(), json!(10));

    let mut obj = Map::new();
    obj.insert("var1".to_owned(), json!(1));
    obj.insert("var2".to_owned(), json!(42));
    metadata.insert("obj".to_owned(), json!(obj));

    let context = ContextJson {
        past: serde_json::json!({}),
        current: serde_json::json!({}),
        metadata: json!(metadata),
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
        flow, step_name, context, &event, None, None, None,
    )));
}

fn main() {
    let flow = read_file("CSML/test.csml".to_owned()).unwrap();

    // let flow = match parse_file(&text) {
    //     Ok(flow) => flow,
    //     Err(e) => panic!("{:?}", e),
    // };

    interpret_flow(&flow, "start");
}

// ((1 + 3) > 6) && var
// if (true == (7 && (7 - (3+(2*1)) == 2)))) {
//     goto toto
// }
// "{{(1 + 3)}} {{var}}"
// ((1 + 3) > 6) + var
