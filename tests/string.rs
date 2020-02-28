mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file, gen_event};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/stdlib/string.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn string_step_0() {
    let data = r#"{
        "memories":[
            {"key":"s", "value":"Hello "},
            {"key":"s", "value":"Hello World"},
            {"key":"s", "value":"HELLO WORLD"},
            {"key":"s", "value":"hello world"}
        ],
        "messages":[
            {"content":{"text": "Hello World"}, "content_type":"text"},
            {"content":{"text": "HELLO WORLD"}, "content_type":"text"},
            {"content":{"text": null}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_0");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_1() {
    let data = r#"{
        "memories":[
            {"key":"s", "value":"Hello"}
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_2() {
    let data = r#"{
        "memories":[
            {"key":"s", "value":"Hello"}
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "false"}, "content_type":"text"},
            {"content":{"text": "false"}, "content_type":"text"},
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
