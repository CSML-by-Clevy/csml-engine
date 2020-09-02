mod support;

use csml_interpreter::data::context::ContextJson;
use csml_interpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_regex_0() {
    let data = r#"{"memories":[{"key":"var", "value":"Hello"}], "messages":[{"content":["H"], "content_type":"array"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_1() {
    let data = r#"{"memories":[{"key":"var", "value":"hello"}], "messages":[{"content":{"text":null}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_2() {
    let data = r#"{"memories":[{"key":"var", "value":"Hello World"}], "messages":[{"content":["H", "W"], "content_type":"array"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_2",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_3",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn ok_regex_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_4",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn ok_regex_5() {
    let data = r#"{"memories":[{"key":"var", "value":"Batman"}], "messages":[{"content":["Bat"], "content_type":"array"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_5",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_6() {
    let data = r#"{"memories":[{"key":"var", "value":"Ceci est une question ? Oui ou non"}], "messages":[{"content":{"text": "true"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_6",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_7() {
    let data = r#"{"memories":[{"key":"var", "value":"0123456789"}], "messages":[{"content":["0", "1", "2", "3"], "content_type":"array"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_7",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_8() {
    let data = r#"{"memories":[{"key":"var", "value":"Hel14lo"}], "messages":[{"content":["1", "4"], "content_type":"array"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "regex_8",
            "flow",
        ),
        "CSML/basic_test/stdlib/regex.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
