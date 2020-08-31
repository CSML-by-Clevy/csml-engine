mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn event_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "content"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new(
            "content_type",
            "content",
            serde_json::Value::Object(serde_json::Map::new()),
        ),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_0",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
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

        ]}"#;

    let mut map = serde_json::Map::new();
    let mut other_map = serde_json::Map::new();

    other_map.insert(
        "yolo".to_owned(),
        serde_json::Value::String("my name is yolo".to_owned()),
    );
    map.insert("toto".to_owned(), serde_json::Value::Object(other_map));

    let msg = format_message(
        Event::new("content_type", "content", serde_json::Value::Object(map)),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_1",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
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
        ]}"#;
    let msg = format_message(
        Event::new(
            "content_type",
            "content",
            serde_json::Value::Object(serde_json::Map::new()),
        ),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_2",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
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
        ]}"#;
    let msg = format_message(
        Event::new(
            "content_type",
            "content",
            serde_json::Value::Object(serde_json::Map::new()),
        ),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_3",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
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
        ]}"#;

    let mut map = serde_json::Map::new();

    map.insert(
        "text".to_owned(),
        serde_json::Value::String("42".to_owned()),
    );

    let msg = format_message(
        Event::new("content_type", "content", serde_json::Value::Object(map)),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_4",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn event_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text":"true"}, "content_type":"text"}
        ]}"#;

    let mut map = serde_json::Map::new();

    map.insert(
        "text".to_owned(),
        serde_json::Value::String("hola@toto.com".to_owned()),
    );

    let msg = format_message(
        Event::new("content_type", "content", serde_json::Value::Object(map)),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_5",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn event_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text":"true"}, "content_type":"text"}
        ]}"#;

    let mut map = serde_json::Map::new();

    map.insert("text".to_owned(), serde_json::Value::String("a".to_owned()));

    let msg = format_message(
        Event::new("content_type", "content", serde_json::Value::Object(map)),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_6",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn event_step_7() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text":"true"}, "content_type":"text"}
        ]}"#;

    let mut map = serde_json::Map::new();

    map.insert("text".to_owned(), serde_json::Value::String("a".to_owned()));

    let msg = format_message(
        Event::new("content_type", "content", serde_json::Value::Object(map)),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_7",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn event_step_8() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text":"false"}, "content_type":"text"}
        ]}"#;

    let mut map = serde_json::Map::new();

    map.insert("text".to_owned(), serde_json::Value::String("b".to_owned()));

    let msg = format_message(
        Event::new("content_type", "content", serde_json::Value::Object(map)),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_8",
            "flow",
        ),
        "CSML/basic_test/event.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
