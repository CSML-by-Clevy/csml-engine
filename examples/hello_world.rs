// use csmlinterpreter::{
//     data::{ContextJson, Event, MessageData},
//     interpret,
// };
// use serde_json::{json, map::Map, Value};

// use std::fs::File;
// use std::io::prelude::*;

// fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
//     let mut file = File::open(file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// pub fn format_message(result: MessageData) -> Value {
//     let mut message: Map<String, Value> = Map::new();
//     let mut vec = vec![];
//     let mut memories = vec![];

//     for msg in result.messages.iter() {
//         vec.push(msg.to_owned().message_to_json());
//     }

//     if let Some(mem) = result.memories {
//         for elem in mem.iter() {
//             let mut map = Map::new();
//             map.insert(elem.key.to_owned(), elem.value.to_owned());
//             memories.push(json!(map));
//         }
//     }

//     message.insert("memories".to_owned(), Value::Array(memories));
//     message.insert("messages".to_owned(), Value::Array(vec));
//     message.insert(
//         "next_flow".to_owned(),
//         match serde_json::to_value(result.next_flow) {
//             Ok(val) => val,
//             _ => json!(null),
//         },
//     );
//     message.insert(
//         "next_step".to_owned(),
//         match serde_json::to_value(result.next_step) {
//             Ok(val) => val,
//             _ => json!(null),
//         },
//     );
//     Value::Object(message)
// }

// fn interpret_flow(flow: &str, step_name: &str) {
//     let event = Event::text("hello");
//     let mut metadata = Map::new();

//     metadata.insert("firstname".to_owned(), json!("Alexis"));

//     metadata.insert("mavar".to_owned(), json!(10));

//     let mut obj = Map::new();
//     obj.insert("var1".to_owned(), json!(1));
//     obj.insert("var2".to_owned(), json!(42));
//     metadata.insert("obj".to_owned(), json!(obj));

//     let context = ContextJson {
//         current: serde_json::json!({}),
//         metadata: json!(metadata),
//         api_info: None,
//         hold: None,
//     };

//     dbg!(format_message(interpret(
//         flow, step_name, context, &event, None,
//     )));
// }

// fn main() {
//     let flow = read_file("CSML/examples/hello_world.csml".to_owned()).unwrap();

//     interpret_flow(&flow, "start");
// }
