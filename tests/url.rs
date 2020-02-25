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
fn ok_url() {
    let data = r#"{"messages":[ {"content":{ "url": "test", "text": "test", "title": "test"},"content_type":"url"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "url", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_url_step1() {
    let data = r#"{"messages":[ {"content":{ "url": "test", "text": "test", "title": "test" },"content_type":"url"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "url", "url1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_url_step2() {
    let data = r#"{"messages":[ {"content":{ "url": "test", "text": "plop", "title": "test" },"content_type":"url"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "url", "url2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_url_step3() {
    let data = r#"{"messages":[ {"content":{ "url": "test", "text": "plop", "title": "rand" },"content_type":"url"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "url", "url3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
