mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_json_object_step1() {
    let data = r#"{"messages":[ {"content":{"text":"1"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step1",
            "flow",
        ),
        "CSML/basic_test/json_object.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_json_object_step2() {
    let data = r#"{"messages":[ {"content":{"text":"4"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step2",
            "flow",
        ),
        "CSML/basic_test/json_object.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_json_object_step3() {
    let data =
        r#"{"messages":[ {"content":{"text":"true"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step3",
            "flow",
        ),
        "CSML/basic_test/json_object.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
