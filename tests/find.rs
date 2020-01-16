mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use std::collections::HashMap
;use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str) -> MessageData {
    let text = read_file("CSML/built-in/find.csml".to_owned()).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let mut context = gen_context(HashMap::new(), HashMap::new(), HashMap::new(), 0, false);

    interpret(&flow, step, &mut context, &event, None, None, None)
}

#[test]
fn ok_find() {
    let data = r#"{"messages":[ {"content":{ "text": "true"  },"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_find_step1() {
    let data = r#"{"messages":[ {"content":{ "text": "false" },"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "find1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
