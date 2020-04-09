mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_length() {
    let data =
        r#"{"messages":[ {"content":{ "text": "5"  },"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "start",
            "flow",
        ),
        "CSML/basic_test/built-in/length.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_length_1() {
    let data =
        r#"{"messages":[ {"content":{ "text": "2"  },"content_type":"text"} ],"memories":[]}"#;
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
        "CSML/basic_test/built-in/length.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_length_2() {
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
        "CSML/basic_test/built-in/length.csml",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}
