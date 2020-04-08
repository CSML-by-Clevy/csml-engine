mod support;

use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_length() {
    let data = r#"{"messages":[ {"content":{ "text": "5"  },"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/length.csml",
        "start",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_length_1() {
    let data = r#"{"messages":[ {"content":{ "text": "2"  },"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/length.csml",
        "step_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_length_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/length.csml",
        "step_1",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}
