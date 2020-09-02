mod support;

use csml_interpreter::data::context::ContextJson;
use csml_interpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn string_step_0() {
    let data = r#"{
        "memories":[
            {"key":"s", "value":"Hello "},
            {"key":"s", "value":"Hello World"}
        ],
        "messages":[
            {"content":{"text": "Hello World"}, "content_type":"text"},
            {"content":{"text": "HELLO WORLD"}, "content_type":"text"},
            {"content":{"text": "hello world"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
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
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
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
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_2",
            "flow",
        ),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
