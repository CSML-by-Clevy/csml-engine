mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_jsonvalue, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/stdlib/type_of.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn ok_type_of_array() {
    let data = r#"{"memories":[{"key":"var", "value":[]}], "messages":[{"content":{"text":"array"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "array");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_boolean() {
    let data = r#"{"memories":[{"key":"var", "value":true}], "messages":[{"content":{"text":"boolean"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "boolean");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_float() {
    let data = r#"{"memories":[{"key":"var", "value":0.42}], "messages":[{"content":{"text":"float"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "float");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_int() {
    let data = r#"{"memories":[{"key":"var", "value":0}], "messages":[{"content":{"text":"int"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "int");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_null() {
    let data = r#"{"memories":[{"key":"var", "value":null}], "messages":[{"content":{"text":"Null"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "_null");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_object() {
    let data = r#"{"memories":[{"key":"var", "value":{}}], "messages":[{"content":{"text":"object"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "object");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_string() {
    let data = r#"{"memories":[{"key":"var", "value":""}], "messages":[{"content":{"text":"string"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "string");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
