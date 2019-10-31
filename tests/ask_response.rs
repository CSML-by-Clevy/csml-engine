mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use multimap::MultiMap;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, name: &str) -> MessageData {
    let file = format!("CSML/ask_response/{}", name);
    let text = read_file(file).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let memory = gen_context(MultiMap::new(), MultiMap::new(), MultiMap::new(), 0, false);

    interpret(&flow, "start", &memory, &event)
}

#[test]
fn ok_ask_normal() {
    let data = r#"{"messages":[{"content":"ask","content_type":"text"}],"next_flow":null,"memories":[ {"key":"myvar", "value": 4} ],"next_step":"end"}"#;
    let msg = format_message(None, "normal.csml");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_response_normal() {
    let data = r#"{"messages":[{"content":"response","content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(Some(gen_event("22")), "normal.csml");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_ask_short() {
    let data = r#"{"messages":[{"content":"ask","content_type":"text"}],"next_flow":null,"memories":[],"next_step":null}"#;
    let msg = format_message(None, "short.csml");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_response_short() {
    let data = r#"{"messages":[{"content":"response","content_type":"text"}],"next_flow":null,"memories":[{"key":"myvar", "value": "22"}],"next_step":null}"#;
    let msg = format_message(Some(gen_event("22")), "short.csml");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
