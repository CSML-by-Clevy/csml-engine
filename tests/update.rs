mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use std::collections::HashMap;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str) -> MessageData {
    let text = read_file("CSML/update.csml".to_owned()).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let mut context = gen_context(HashMap::new(), HashMap::new(), HashMap::new(), 0, false);

    interpret(&flow, step, &mut context, &event, None, None, None)
}

#[test]
fn ok_update_step1() {
    let data = r#"{"messages":[ {"content":{"text":"1"},"content_type":"text"}, {"content":{"text":"4"},"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "step1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_update_step2() {
    let data = r#"{"messages":[ {"content": [{"test": 1}, 2, 3, 4, 5], "content_type":"array"}, {"content": [1, 2, 3, 4, 5], "content_type":"array"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "step2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_update_step3() {
    let data = r#"{"messages":[ {"content": [1], "content_type":"array"}, {"content": [2], "content_type":"array"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "step3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_update_step4() {
    let data = r#"{"messages":[ {"content": [1], "content_type":"array"}, {"content": [1, 2], "content_type":"array"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "step4");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_update_step5() {
    let data = r#"{"messages":[ {"content": [1, 2], "content_type":"array"}, {"content": [1], "content_type":"array"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "step5");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
