mod support;

use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn array_step_0() {
    let data =
        r#"{"memories":[{"key":"vec", "value":[]}, {"key":"vec", "value": [42]}], "messages":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_0",
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
        ],
        "next_flow":null, "next_step":"end"
    }
    "#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_1",
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
            "messages":[{"content":{"text": "false"},"content_type":"text"}, {"content":{"text": "true"}, "content_type":"text"}],
            "next_flow":null,
            "next_step":"end"
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_2",
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
            "messages":[{"content":{"text": "2"}, "content_type":"text"}, {"content":{"text": "24"}, "content_type":"text"}, {"content":{"text": "42"}, "content_type":"text"}],
            "next_flow":null,
            "next_step":"end"
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_3",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_4",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_5",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_6",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_7() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_7",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_8() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_8",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn array_step_9() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":""}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "step_9",
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
        "CSML/basic_test/stdlib/array.csml",
        "step_10",
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
        "CSML/basic_test/stdlib/array.csml",
        "step_11",
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
        "CSML/basic_test/stdlib/array.csml",
        "array_index_of_0",
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
        "CSML/basic_test/stdlib/array.csml",
        "array_index_of_1",
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
        "CSML/basic_test/stdlib/array.csml",
        "array_index_of_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_find_0() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":null}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/stdlib/array.csml",
        "array_find_0",
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
        "CSML/basic_test/stdlib/array.csml",
        "array_find_1",
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
        "CSML/basic_test/stdlib/array.csml",
        "array_find_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
