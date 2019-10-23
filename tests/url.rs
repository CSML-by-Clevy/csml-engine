mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use multimap::MultiMap;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, file: &str, step: &str) -> MessageData {
    let text = read_file(format!("CSML/built-in/{}.csml", file)).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let memory = gen_context(MultiMap::new(), MultiMap::new(), MultiMap::new(), 0, false);

    interpret(&flow, step, &memory, &event)
}

#[test]
fn ok_url() {
    let data = r#"{"messages":[ {"content":{ "url": {"url": "test", "text": "test", "title": "test"} },"content_type":"url"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "url", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_url_step1() {
    let data = r#"{"messages":[ {"content":{ "url": {"url": "test", "text": "test", "title": "test"} },"content_type":"url"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "url", "url1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_url_step2() {
    let data = r#"{"messages":[ {"content":{ "url": {"url": "test", "text": "plop", "title": "test"} },"content_type":"url"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "url", "url2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_url_step3() {
    let data = r#"{"messages":[ {"content":{ "url": {"url": "test", "text": "plop", "title": "rand"} },"content_type":"url"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "url", "url3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}


#[test]
fn ok_video() {
    let data = r#"{"messages":[ {"content":{ "video": {"url": "test"} },"content_type":"video"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "video", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_video_step2() {
    let data = r#"{"messages":[ {"content":{ "video": {"url": "test", "service": "youtube"} },"content_type":"video"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "video", "video1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_video_step3() {
    let data = r#"{"messages":[ {"content":{ "video": {"url": "test", "service": "youtube"} },"content_type":"video"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "video", "video2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_audio() {
    let data = r#"{"messages":[ {"content":{ "audio": {"url": "test"} },"content_type":"audio"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "audio", "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_audio_step2() {
    let data = r#"{"messages":[ {"content":{ "audio": {"url": "test", "service": "youtube"} },"content_type":"audio"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "audio", "audio1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_audio_step3() {
    let data = r#"{"messages":[ {"content":{ "audio": {"url": "test", "service": "youtube"} },"content_type":"audio"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "audio", "audio2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
