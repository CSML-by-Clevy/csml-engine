mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{data::*, message::MessageData};
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str) -> MessageData {
    let text = read_file("CSML/object.csml".to_owned()).unwrap();

    let context = gen_context(
        serde_json::json!({}),
        serde_json::json!({}),
        serde_json::json!({}),
        0,
        false,
    );

    interpret(&text, step, context, &event, None, None, None)
}

fn check_error_component(vec: &MessageData) -> bool {
    let comp = &vec.messages[0];
    comp.content_type == "error"
}

#[test]
fn ok_object_step1() {
    let data = r#"{"messages":[ {"content":{"text":"1"},"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "step1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_object_step2() {
    let data = r#"{"messages":[ {"content":{"text":"4"},"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "step2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_object_step3() {
    let data = r#"{"messages":[ {"content":{"text":"true"},"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None, "step3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_object_step4() {
    let msg = format_message(None, "step4");
    let res = check_error_component(&msg);

    assert_eq!(res, false)
}

#[test]
fn ok_object_step5() {
    let msg = format_message(None, "step5");
    let v: Value = message_to_jsonvalue(msg);

    let int = v["messages"][0]["content"]["text"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    if int < 1 && int > 5 {
        panic!("Random fail {}", int);
    }
}
