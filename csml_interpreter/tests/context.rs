mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

////////////////////////////////////////////////////////////////////////////////
/// VALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn ok_current_step() {
    let data = r#"{"messages":[ {"content":{"text": "current_step"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "current_step",
            "flow",
            None,
        ),
        "CSML/basic_test/context.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_current_flow() {
    let data =
        r#"{"messages":[ {"content":{"text": "flow"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "current_flow",
            "flow",
            None,
        ),
        "CSML/basic_test/context.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_default_flow() {
    let data =
        r#"{"messages":[ {"content":{"text": "flow"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "default_flow",
            "flow",
            None,
        ),
        "CSML/basic_test/context.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
