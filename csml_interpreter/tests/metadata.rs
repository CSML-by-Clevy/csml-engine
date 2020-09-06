mod support;

use csml_interpreter::data::context::ContextJson;
use csml_interpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn metadata() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content":{"var": 42}, "content_type":"object"}
        ]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({"var": 42}),
            None,
            None,
            "start",
            "flow",
        ),
        "CSML/basic_test/metadata.csml",
    );

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
        ]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({"var": 42}),
            None,
            None,
            "step1",
            "flow",
        ),
        "CSML/basic_test/metadata.csml",
    );

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
        ]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({"var": 42}),
            None,
            None,
            "step2",
            "flow",
        ),
        "CSML/basic_test/metadata.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
