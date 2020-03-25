mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/stdlib/regex.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn ok_regex_0() {
    let data = r#"{"memories":[{"key":"var", "value":"Hello"}], "messages":[{"content":["H"], "content_type":"array"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "regex_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_1() {
    let data = r#"{"memories":[{"key":"var", "value":"hello"}], "messages":[{"content":{"text":null}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "regex_1");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_2() {
    let data = r#"{"memories":[{"key":"var", "value":"Hello World"}], "messages":[{"content":["H", "W"], "content_type":"array"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "regex_2");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_3() {
    let data = r#"{"memories":[], "messages":[{"content":{"error":"usage: parameter must be of type string at line 21, column 13"}, "content_type":"error"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "regex_3");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_4() {
    let data = r#"{"memories":[], "messages":[{"content":{"error":"usage: match(Primitive<String>) at line 26, column 13"}, "content_type":"error"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "regex_4");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_5() {
    let data = r#"{"memories":[{"key":"var", "value":"Batman"}], "messages":[{"content":["Bat"], "content_type":"array"}], "next_flow":null, "next_step":"end"}"#;

    let msg = format_message(gen_event(""), "regex_5");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_6() {
    let data = r#"{"memories":[{"key":"var", "value":"Ceci est une question ? Oui ou non"}], "messages":[{"content":{"text": "true"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;

    let msg = format_message(gen_event(""), "regex_6");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_7() {
    let data = r#"{"memories":[{"key":"var", "value":"0123456789"}], "messages":[{"content":["0", "1", "2", "3"], "content_type":"array"}], "next_flow":null, "next_step":"end"}"#;

    let msg = format_message(gen_event(""), "regex_7");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_regex_8() {
    let data = r#"{"memories":[{"key":"var", "value":"Hel14lo"}], "messages":[{"content":["1", "4"], "content_type":"array"}], "next_flow":null, "next_step":"end"}"#;

    let msg = format_message(gen_event(""), "regex_8");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
