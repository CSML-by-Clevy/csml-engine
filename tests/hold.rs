mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use std::collections::HashMap;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str, instruction_index: Option<usize>) -> MessageData {
    let text = read_file("CSML/hold.csml".to_owned()).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let mut context = gen_context(HashMap::new(), HashMap::new(), HashMap::new(), 0, false);

    interpret(&flow, step, &mut context, &event, None, instruction_index, None)
}

#[test]
fn hold_test_none() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}, {"content":{"text":"2"}, "content_type":"text"}, {"content":{"text":"4"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "start", None);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_0() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}, {"content":{"text":"2"}, "content_type":"text"}, {"content":{"text":"4"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "start", Some(0));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_5() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"4"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "start", Some(5));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_12() {
    let data = r#"{"memories":[], "messages":[], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "start", Some(12));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_14() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"3"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "start", Some(14));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_17() {
    let data = r#"{"memories":[], "messages":[], "next_flow":null, "next_step":null}"#;
    let msg = format_message(None, "start", Some(17));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
