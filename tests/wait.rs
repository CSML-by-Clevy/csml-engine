mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file, gen_event};

fn format_message(event: Event, file: &str, step: &str) -> MessageData {
    let text = read_file(format!("CSML/basic_test/built-in/{}.csml", file)).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn wait_0() {
    let data = r#"{"messages":[ {"content":{"error": "Builtin Wait expect one argument of type int or float | example: Wait(3) at line 5, column 6"},"content_type":"error"} ],"next_flow":null,"memories":[],"next_step":null}"#;
    let msg = format_message(gen_event(""), "wait", "wait_0");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn wait_1() {
    let data = r#"{"messages":[ {"content":{"duration": 10},"content_type":"wait"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "wait", "wait_1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
