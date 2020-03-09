mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/event.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn event_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "content"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(
        Event {
            content_type: "content_type".to_owned(),
            content: "content".to_owned(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
        },
        "step_0",
    );

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn event_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"yolo": "my name is yolo"}, "content_type":"object"},
            {"content":{"text": "my name is yolo"}, "content_type":"text"}

        ],
        "next_flow":null,
        "next_step":"end"}"#;

    let mut map = serde_json::Map::new();
    let mut other_map = serde_json::Map::new();

    other_map.insert(
        "yolo".to_owned(),
        serde_json::Value::String("my name is yolo".to_owned()),
    );
    map.insert("toto".to_owned(), serde_json::Value::Object(other_map));
    let msg = format_message(
        Event {
            content_type: "content_type".to_owned(),
            content: "content".to_owned(),
            metadata: serde_json::Value::Object(map),
        },
        "step_1",
    );

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn event_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "content_type"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(
        Event {
            content_type: "content_type".to_owned(),
            content: "content".to_owned(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
        },
        "step_2",
    );

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn event_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{}, "content_type":"content_type"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;
    let msg = format_message(
        Event {
            content_type: "content_type".to_owned(),
            content: "content".to_owned(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
        },
        "step_3",
    );

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn event_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text":"true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":"end"}"#;

    let mut map = serde_json::Map::new();

    map.insert(
        "text".to_owned(),
        serde_json::Value::String("42".to_owned()),
    );

    let msg = format_message(
        Event {
            content_type: "content_type".to_owned(),
            content: "content".to_owned(),
            metadata: serde_json::Value::Object(map),
        },
        "step_4",
    );

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
