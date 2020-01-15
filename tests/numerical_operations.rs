mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{
    json_to_rust::*,
    message::{Message, MessageData},
};
use csmlinterpreter::parser::{literal::Literal, Parser};
use std::collections::HashMap;
use serde_json::Value;
use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, name: &str, step: &str) -> MessageData {
    let file = format!("CSML/numerical_operations/{}", name);
    let text = read_file(file).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let mut context = gen_context(HashMap::new(), HashMap::new(), HashMap::new(), 0, false);

    interpret(&flow, step, &mut context, &event, None, 0, None)
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
fn ok_divition2() {
    let data = r#"{"messages":[ {"content":{"text":"21.333333333333332"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "divition.csml", "div2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

fn check_error_component(vec: &[Message]) -> bool {
    let comp = &vec[0];
    match &comp.content {
        Literal::FunctionLiteral { name, .. } if name == "text" => true,
        _ => false,
    }
}

#[test]
fn ok_divition3() {
    let file = format!("CSML/numerical_operations/{}", "divition.csml");
    let text = read_file(file).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let mut context = gen_context(HashMap::new(), HashMap::new(), HashMap::new(), 0, false);

    match &interpret(&flow, "div3", &mut context, &None, None, 0, None) {
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
