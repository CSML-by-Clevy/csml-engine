pub mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, file: &str, step: &str) -> MessageData {
    let text = read_file(format!("CSML/built-in/{}.csml", file)).unwrap();

    let context = gen_context(
        serde_json::json!({}),
        serde_json::json!({}),
    );

    interpret(&text, step, context, &event, None, None, None)
}

#[test]
fn ok_audio() {
    let data = r#"{"messages":[ {"content":{ "url": "test" },"content_type":"audio"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "audio", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_audio_step2() {
    let data = r#"{"messages":[ {"content":{"url": "test", "service": "youtube" },"content_type":"audio"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "audio", "audio1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_audio_step3() {
    let data = r#"{"messages":[ {"content":{ "url": "test", "service": "youtube" },"content_type":"audio"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "audio", "audio2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
