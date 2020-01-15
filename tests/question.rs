mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use std::collections::HashMap;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, file: &str, step: &str) -> MessageData {
    let text = read_file(format!("CSML/built-in/{}.csml", file)).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let mut context = gen_context(HashMap::new(), HashMap::new(), HashMap::new(), 0, false);

    interpret(&flow, step, &mut context, &event, None, 0, None)
}

#[test]
fn ok_question() {
    let data = r#"{"messages":[ {"content": { "buttons": [ {"accepts": ["b1"], "content": {"payload": "b1", "title": "b1"}, "content_type": "button"}, {"accepts": ["b2"], "content": {"payload": "b2", "title": "b2"}, "content_type": "button"}], "title": "title"},"content_type":"question"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "question", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_question_step1() {
    let data = r#"{"messages":[ {"content": { "buttons": [ {"accepts": ["b1"], "content": {"payload": "b1", "title": "b1"}, "content_type": "button"}, {"accepts": ["b2"], "content": {"payload": "b2", "title": "b2"}, "content_type": "button"}], "title": "title"},"content_type":"question"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "question", "question1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_question_step2() {
    let data = r#"{"messages":[ {"content": { "buttons": [ {"accepts": ["b1"], "content": {"payload": "b1", "title": "b1"}, "content_type": "button"}, {"accepts": ["b2"], "content": {"payload": "b2", "title": "b2"}, "content_type": "button"}]},"content_type":"question"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "question", "question2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
