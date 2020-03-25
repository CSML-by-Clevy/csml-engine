mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/stdlib/number.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn int_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1764"}, "content_type":"text"},
            {"content":{"text": "int"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "int_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn int_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "3725.1900894013565"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "int_1");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn int_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "3725.1900894013565"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "int_2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn float_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1764"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "float_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn float_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "3725.1900894013565"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "float_1");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1764"}, "content_type":"text"},
            {"content":{"text": "int"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "string_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "3725.1900894013565"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "string_1");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
