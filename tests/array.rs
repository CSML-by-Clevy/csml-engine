mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_jsonvalue, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/stdlib/array.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn array_step_0() {
    let data = r#"{"memories":[{"key":"vec", "value":[]}, {"key":"vec", "value": [42]}], "messages":[], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_0");

    let v1: Value = message_to_jsonvalue(msg);
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
        "next_flow":null, "next_step":null
    }
    "#;
    let msg = format_message(gen_event(""), "step_1");

    let v1: Value = message_to_jsonvalue(msg);
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
            "next_step":null
        }"#;
    let msg = format_message(gen_event(""), "step_2");

    let v1: Value = message_to_jsonvalue(msg);
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
            "next_step":null
        }"#;
    let msg = format_message(gen_event(""), "step_3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_4() {
    let data = r#"{"memories":[], "messages":[{"content":{"error":"usage: index must be positive at line 40, column 12"}, "content_type":"error"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_4");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_5() {
    let data = r#"{"memories":[], "messages":[{"content":{"error":"usage: index must be lower or equal than array.length() at line 45, column 12"}, "content_type":"error"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_5");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_6() {
    let data = r#"{"memories":[], "messages":[{"content":{"error":"usage: index must be positive at line 50, column 12"}, "content_type":"error"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_6");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_7() {
    let data = r#"{"memories":[], "messages":[{"content":{"error":"usage: index must be lower or equal than array.length() at line 55, column 12"}, "content_type":"error"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_7");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_8() {
    let data = r#"{"memories":[], "messages":[{"content":{"error":"usage: join(Primitive<String>) at line 60, column 13"}, "content_type":"error"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_8");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_9() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":null}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_9");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_10() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_10");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn array_step_11() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1,2"}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "step_11");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
