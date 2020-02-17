pub mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{
    data::*,
    message::{Message, MessageData},
};
use serde_json::Value;
use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, name: &str, step: &str) -> MessageData {
    let file = format!("CSML/numerical_operations/{}", name);
    let text = read_file(file).unwrap();

    let context = gen_context(
        serde_json::json!({}),
        serde_json::json!({}),
        serde_json::json!({}),
        0,
        false,
    );

    interpret(&text, step, context, &event, None, None, None)
}

#[test]
fn ok_addition() {
    let data = r#"{"messages":[ {"content":{"text":"5"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "addition.csml", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_subtraction() {
    let data = r#"{"messages":[ {"content":{"text":"-3"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "subtraction.csml", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_multiplication() {
    let data = r#"{"messages":[ {"content":{"text":"8"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "multiplication.csml", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_divition() {
    let data = r#"{"messages":[ {"content":{"text":"2"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "divition.csml", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_division_2() {
    let data = r#"{"messages":[ {"content":{"text":"21.333333333333332"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "divition.csml", "div2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

fn check_error_component(vec: &[Message]) -> bool {
    let comp = &vec[0];

    return comp.content.is_object();
}

#[test]
fn ok_division_3() {
    let file = format!("CSML/numerical_operations/{}", "divition.csml");
    let text = read_file(file).unwrap();

    let context = gen_context(
        serde_json::json!({}),
        serde_json::json!({}),
        serde_json::json!({}),
        0,
        false,
    );

    match &interpret(&text, "div3", context, &None, None, None, None) {
        MessageData {
            memories: None,
            messages: vec,
            next_flow: None,
            next_step: None,
            instruction_index: 0,
            ..
        } if vec.len() == 1 && check_error_component(&vec) => {}
        e => panic!("Error in div by 0 {:?}", e),
    }
}

#[test]
fn ok_remainder() {
    let data = r#"{"messages":[ {"content":{"text":"2"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "remainder.csml", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_string_to_numeric() {
    let data = r#"{"messages":[ {"content":{"text":"2.5"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "string_to_numeric.csml", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
