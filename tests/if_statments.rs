mod support;

use csmlinterpreter::{interpret};
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use serde_json::Value;
use multimap::MultiMap;

use support::tools::{gen_context, gen_event, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>) -> MessageData {
    let text = read_file("CSML/if_statments.csml".to_owned()).unwrap();
    let flow = Parser::parse_flow(text.as_bytes()).unwrap();

    let memory = gen_context(MultiMap::new(), MultiMap::new(), MultiMap::new(), 0, false);

    interpret(&flow, "start", &memory, &event)
}

#[test]
fn ok_equal_20() {
    let data = r#"{"messages":[{"content":{"text":"event == 20"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(Some(gen_event("20")));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_greater_20() {
    let data = r#"{"messages":[{"content":{"text":"event > 20 && event < 40"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(Some(gen_event("22")));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_greater_equal_50() {
    let data = r#"{"messages":[{"content":{"text":"event >= 50"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(Some(gen_event("50")));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_less_20() {
    let data = r#"{"messages":[{"content":{"text":"event < 20"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(Some(gen_event("4")));


    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}


#[test]
fn ok_less_equal_45() {
    let data = r#"{"messages":[{"content":{"text":"event <= 45"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(Some(gen_event("42")));

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_not_int1() {
    let msg = format_message(Some(gen_event("plop")));
    let data = r#"{"messages":[{"content":{"text":"event is not int"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_not_int2() {
    let msg = format_message(None);
    let data = r#"{"messages":[{"content":{"text":"event is not int"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}