mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_type_of_array() {
    let data = r#"{"memories":[{"key":"var", "value":[]}], "messages":[{"content":{"text":"array"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "array",
            "flow",
            None,
        ),
        "CSML/basic_test/stdlib/type_of.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_boolean() {
    let data = r#"{"memories":[{"key":"var", "value":true}], "messages":[{"content":{"text":"boolean"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "boolean",
            "flow",
            None,
        ),
        "CSML/basic_test/stdlib/type_of.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_float() {
    let data = r#"{"memories":[{"key":"var", "value":0.42}], "messages":[{"content":{"text":"float"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "float",
            "flow",
            None,
        ),
        "CSML/basic_test/stdlib/type_of.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_int() {
    let data = r#"{"memories":[{"key":"var", "value":0}], "messages":[{"content":{"text":"int"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "int",
            "flow",
            None,
        ),
        "CSML/basic_test/stdlib/type_of.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_null() {
    let data = r#"{"memories":[{"key":"var", "value":null}], "messages":[{"content":{"text":"Null"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "_null",
            "flow",
            None,
        ),
        "CSML/basic_test/stdlib/type_of.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_object() {
    let data = r#"{"memories":[{"key":"var", "value":{"_content":{}, "_content_type": "object"} }], "messages":[{"content":{"text":"object"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "object",
            "flow",
            None,
        ),
        "CSML/basic_test/stdlib/type_of.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_type_of_string() {
    let data = r#"{"memories":[{"key":"var", "value":""}], "messages":[{"content":{"text":"string"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "string",
            "flow",
            None,
        ),
        "CSML/basic_test/stdlib/type_of.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
