pub mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;
use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

fn format_message(event: Event, name: &str, step: &str) -> MessageData {
    let file = format!("CSML/basic_test/numerical_operation/{}", name);
    let text = read_file(file).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn ok_string_to_numeric() {
    let data = r#"{"messages":[ {"content":{"text":"2.5"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "string_to_numeric.csml", "start");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
