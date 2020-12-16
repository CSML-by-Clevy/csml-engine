mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn array_step_0() {
    let data =
        r#"{"memories":[{"key":"vec", "value":[]}, {"key":"vec", "value": [42]}], "messages":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_1() {
    let data = r#"
    {
        "memories":[{"key":"vec", "value": [42]}, {"key":"vec", "value": []}],
        "messages":[
            {"content":{"text": "42"}, "content_type":"text"},
            {"content":[], "content_type":"array"}
        ]
    }
    "#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_2() {
    let data = r#"
        {
            "memories":[{"key":"vec", "value": [42]}, {"key":"vec", "value": []}],
            "messages":[{"content":{"text": "false"},"content_type":"text"}, {"content":{"text": "true"}, "content_type":"text"}]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_2",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_3() {
    let data = r#"
        {
            "memories":[{"key":"vec", "value": [42]}, {"key":"vec", "value": [24, 42]}, {"key":"vec", "value": [42]}],
            "messages":[{"content":{"text": "2"}, "content_type":"text"}, {"content":{"text": "24"}, "content_type":"text"}, {"content":{"text": "42"}, "content_type":"text"}]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_3",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_4",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_5",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    eprintln!("=> {:?}", msg.messages);
    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_6",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_7() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_7",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_8() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_8",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_9() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":""}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_9",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_10() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_10",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_11() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1,2"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_11",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_index_of_0() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"-1"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "array_index_of_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_index_of_1() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "array_index_of_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_index_of_2() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"2"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "array_index_of_2",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_find_0() {
    let data = r#"{"memories":[], "messages":[{"content":[], "content_type":"array"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "array_find_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_find_1() {
    let data = r#"{"memories":[], "messages":[{"content":[2, 2], "content_type":"array"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "array_find_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_find_2() {
    let data =
        r#"{"memories":[], "messages":[{"content":[{"obj":"42"}], "content_type":"array"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "array_find_2",
            "flow",
        ),
        "CSML/basic_test/stdlib/array.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
