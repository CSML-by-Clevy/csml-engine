pub mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_jsonvalue, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/built-in/length.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn ok_length() {
    let data = r#"{"messages":[ {"content":{ "text": "5"  },"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_length_1() {
    let data = r#"{"messages":[ {"content":{ "text": "2"  },"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "step_0");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_length_2() {
    let data = r#"{"messages":[ {"content":{ "error": "ERROR: Builtin Length expect one value of type Array or String | example: Length( value ) at line 10, column 6" },"content_type":"error"} ],"next_flow":null,"memories":[],"next_step":null}"#;
    let msg = format_message(gen_event(""), "step_1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
