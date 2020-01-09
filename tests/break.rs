mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use multimap::MultiMap;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str) -> MessageData {
    let text = read_file("CSML/break.csml".to_owned()).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let mut context = gen_context(MultiMap::new(), MultiMap::new(), MultiMap::new(), 0, false);

    interpret(&flow, step, &mut context, &event, None, None)
}

#[test]
fn break_test_0() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(None, "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_1() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(None, "break_test_0");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_2() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(None, "break_test_1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_3() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}, {"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type": "text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(None, "break_test_2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_4() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}], "next_flow":null, "next_step":"foo"}"#;
    let msg = format_message(None, "break_test_3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}