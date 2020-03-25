mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/metadata.csml".to_owned()).unwrap();
    let context = gen_context(serde_json::json!({}), serde_json::json!({"var": 42}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn metadata() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content":{"var": 42}, "content_type":"object"}
        ],
        "next_flow":null,
        "next_step":"end"
    }"#;
    let msg = format_message(gen_event(""), "start");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();
    assert_eq!(v1, v2)
}

#[test]
fn metadata_step1() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content": {"text": "42" }, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"
    }"#;
    let msg = format_message(gen_event(""), "step1");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn metadata_step2() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content": {"text": "42" }, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"
    }"#;
    let msg = format_message(gen_event(""), "step2");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
