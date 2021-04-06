mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_text() {
    let data =
        r#"{"messages":[ {"content":{"text": "Hola"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "start", "flow"),
        "CSML/basic_test/built-in/text.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_text_step1() {
    let data =
        r#"{"messages":[ {"content":{"text": "Hola"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "text1", "flow"),
        "CSML/basic_test/built-in/text.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_text_step2() {
    let data = r#"{"messages":[ {"content":{"text": ""},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "text2", "flow"),
        "CSML/basic_test/built-in/text.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_text_step3() {
    let data = r#"{"messages":[ {"content":{"text": null},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "text3", "flow"),
        "CSML/basic_test/built-in/text.csml",
    );

    let v1: Value = message_to_json_value(msg);

    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_text_step4() {
    let data = r#"
        {
            "messages":[
                {"content":{"error": "< hola > is used before it was saved in memory at line 18, column 17 at flow [flow]"}, "content_type": "error"},
                {"content":{"text": "😀 Null"},"content_type":"text"}
            ],"memories":[]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "text4", "flow"),
        "CSML/basic_test/built-in/text.csml",
    );

    let v1: Value = message_to_json_value(msg);

    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
