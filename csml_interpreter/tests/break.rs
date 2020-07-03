mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn break_test_0() {
    let data =
        r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}]}"#;
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
        "CSML/basic_test/break.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_1() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "break_test_0",
            "flow",
        ),
        "CSML/basic_test/break.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_2() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "break_test_1",
            "flow",
        ),
        "CSML/basic_test/break.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_3() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}, {"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type": "text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "break_test_2",
            "flow",
        ),
        "CSML/basic_test/break.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_4() {
    let data =
        r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "break_test_3",
            "flow",
        ),
        "CSML/basic_test/break.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
