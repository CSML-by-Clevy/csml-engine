mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::{event::Event, primitive::PrimitiveInt, Interval};
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn forget_all() {
    let data = r#"{
        "memories":[],
        "messages":[]
    }"#;
    let mut metadata = HashMap::new();
    metadata.insert(
        "var".to_owned(),
        PrimitiveInt::get_literal(42, Interval::default()),
    );

    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            metadata,
            None,
            None,
            "forget_all",
            "flow",
            None,
        ),
        "CSML/basic_test/syntax/forget.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();
    assert_eq!(v1, v2)
}

#[test]
fn forget_single() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content": {"mem": 42 }, "content_type": "object"}
        ]
    }"#;

    let mut metadata = HashMap::new();
    metadata.insert(
        "var".to_owned(),
        PrimitiveInt::get_literal(42, Interval::default()),
    );

    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            metadata,
            None,
            None,
            "forget_single",
            "flow",
            None,
        ),
        "CSML/basic_test/syntax/forget.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn forget_list() {
    let data = r#"{
        "memories":[],
        "messages":[]
    }"#;
    let mut metadata = HashMap::new();
    metadata.insert(
        "var".to_owned(),
        PrimitiveInt::get_literal(42, Interval::default()),
    );

    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            metadata,
            None,
            None,
            "forget_list",
            "flow",
            None,
        ),
        "CSML/basic_test/syntax/forget.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();
    assert_eq!(v1, v2)
}
