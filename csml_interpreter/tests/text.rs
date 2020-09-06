mod support;

use csml_interpreter::data::context::ContextJson;
use csml_interpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_text() {
    let data =
        r#"{"messages":[ {"content":{"text": "Hola"},"content_type":"text"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "start",
            "flow",
        ),
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
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "text1",
            "flow",
        ),
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
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "text2",
            "flow",
        ),
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
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "text3",
            "flow",
        ),
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
                {"content":{"error": "< hola > is not set in memory at line 18, column 17 in step [text4] from flow [flow]"}, "content_type": "error"},
                {"content":{"text": "😀 Null"},"content_type":"text"}
            ],"memories":[]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "text4",
            "flow",
        ),
        "CSML/basic_test/built-in/text.csml",
    );

    let v1: Value = message_to_json_value(msg);

    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
