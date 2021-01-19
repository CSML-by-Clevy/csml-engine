mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::{event::Event, primitive::PrimitiveInt, Interval};
use std::collections::HashMap;

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
    let mut metadata = HashMap::new();
    metadata.insert(
        "var".to_owned(),
        PrimitiveInt::get_literal(42, Interval::default()),
    );

    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), metadata, None, None, "start", "flow"),
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

    let mut metadata = HashMap::new();
    metadata.insert(
        "var".to_owned(),
        PrimitiveInt::get_literal(42, Interval::default()),
    );

    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), metadata, None, None, "step1", "flow"),
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
    let mut metadata = HashMap::new();
    metadata.insert(
        "var".to_owned(),
        PrimitiveInt::get_literal(42, Interval::default()),
    );

    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), metadata, None, None, "step2", "flow"),
        "CSML/basic_test/metadata.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
