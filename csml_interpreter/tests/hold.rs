mod support;

use csml_interpreter::data::event::Event;
use csml_interpreter::data::hold::Hold;
use csml_interpreter::data::ContextJson;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn hold_test_none() {
    let data = r#"
    {"memories":[],
    "messages":[
        {"content":{"error":"< this_hold > is not in in memory at line 2, column 5 in step [start] from flow [flow]"}, "content_type":"error"},
        {"content":{"text":"1"}, "content_type":"text"},
        {"content":{"text":"2"}, "content_type":"text"},
        {"content":{"error": "< this_hold > is not in in memory at line 8, column 6 in step [start] from flow [flow]"}, "content_type":"error"},
        {"content":{"text":"4"}, "content_type":"text"}]
    }
    "#;
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
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_0() {
    let data = r#"
    {
        "memories":[],
        "messages":[
            {"content":{"text":"1"}, "content_type":"text"},
            {"content":{"text":"2"}, "content_type":"text"},
            {"content":{"error": "< this_hold > is not in in memory at line 8, column 6 in step [start] from flow [flow]"}, "content_type":"error"},
            {"content":{"text":"4"}, "content_type":"text"}
        ]
    }
    "#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            Some(Hold::new(0, serde_json::json!({}))),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_5() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"4"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            Some(Hold::new(5, serde_json::json!({}))),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_12() {
    let data = r#"{"memories":[], "messages":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            Some(Hold::new(12, serde_json::json!({}))),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_14() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"3"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            Some(Hold::new(14, serde_json::json!({}))),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_17() {
    let data = r#"{"memories":[], "messages":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            Some(Hold::new(17, serde_json::json!({}))),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
