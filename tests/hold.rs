mod support;

use csmlinterpreter::data::{Event, Hold, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

fn format_message(event: Event, step: &str, instruction_index: Option<usize>) -> MessageData {
    let text = read_file("CSML/basic_test/hold.csml".to_owned()).unwrap();
    // instruction_index
    let mut context = gen_context(serde_json::json!({}), serde_json::json!({}));
    if let Some(index) = instruction_index {
        context.hold = Some(Hold {
            index,
            step_vars: serde_json::json!({}),
        });
    };
    interpret(&text, step, context, &event, None)
}

#[test]
fn hold_test_none() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}, {"content":{"text":"2"}, "content_type":"text"}, {"content":{"text":"4"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "start", None);

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_0() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"1"}, "content_type":"text"}, {"content":{"text":"2"}, "content_type":"text"}, {"content":{"text":"4"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "start", Some(0));

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_5() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"4"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "start", Some(5));

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_12() {
    let data = r#"{"memories":[], "messages":[], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "start", Some(12));

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_14() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"3"}, "content_type":"text"}], "next_flow":null, "next_step":"end"}"#;
    let msg = format_message(gen_event(""), "start", Some(14));

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_17() {
    let data = r#"{"memories":[], "messages":[], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "start", Some(17));

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
