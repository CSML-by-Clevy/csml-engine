mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_find() {
    let data =
        r#"{"messages":[ {"content":{ "text": "true"  },"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "start",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/find.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_find_step1() {
    let data =
        r#"{"messages":[ {"content":{ "text": "false" },"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "find1",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/find.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
