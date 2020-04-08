mod support;

use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_audio() {
    let data =
        r#"{"messages":[ {"content":{ "url": "test" },"content_type":"audio"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/audio.csml",
        "start",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_audio_step2() {
    let data = r#"{"messages":[ {"content":{"url": "test", "service": "youtube" },"content_type":"audio"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/audio.csml",
        "audio1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_audio_step3() {
    let data = r#"{"messages":[ {"content":{ "url": "test", "service": "youtube" },"content_type":"audio"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        "CSML/basic_test/built-in/audio.csml",
        "audio2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
