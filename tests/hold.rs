mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use std::collections::HashMap;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str, instruction_index: usize) -> MessageData {
    let text = read_file("CSML/hold.csml".to_owned()).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let mut context = gen_context(HashMap::new(), HashMap::new(), HashMap::new(), 0, false);

    interpret(&flow, step, &mut context, &event, None, instruction_index, None)
}

#[test]
fn hold_test_0() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "start", 0);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_1() {
    let data = r#"{"memories":[], "messages":[], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "start", 1);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_2() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"3"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l0", 5);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_3() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"2"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l0", 3);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_4() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l0", 1);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_5() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l0", 0);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_6() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"2"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l0", 2);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_7() {
    let data = r#"{"memories":[], "messages":[], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l0", 6);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_8() {
    let data = r#"{"memories":[], "messages":[], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l1", 11);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_9() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"5"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l1", 10);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_10() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"5"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l1", 9);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_11() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"|2|"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l1", 7);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_12() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"|2|"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l1", 8);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_13() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"3"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l1", 3);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_14() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"3"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l1", 2);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_15() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"3"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l1", 1);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_16() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"0"}, "content_type":"text"}, {"content":{"text":"3"}, "content_type":"text"}, {"content":{"text":"7"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l2", 0);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_17() {
    let data = r#"{"memories":[], "messages":[], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l3", 1);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_18() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(None, "l4", 1);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

