mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{data::*, message::MessageData};
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str) -> MessageData {
    let text = read_file("CSML/stdlib/object.csml".to_owned()).unwrap();

    let context = gen_context(
        serde_json::json!({}),
        serde_json::json!({}),
        serde_json::json!({}),
        0,
        false,
    );

    interpret(&text, step, context, &event, None, None, None)
}

#[test]
fn object_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"error": "usage: key must be of type string at line 9, column 12"}, "content_type":"error"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_0");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_1() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{}},
            {"key":"obj", "value":{"42": "Hello"}}
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_2() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{}},
            {"key":"obj", "value":{}},
            {"key":"obj", "value":{"Hello": 42}},
            {"key":"obj", "value":{}}
        ],
        "messages":[
            {"content":{"text": null}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_3() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{}},
            {"key":"obj", "value":{"42": "Hello"}},
            {"key":"obj", "value":{}}
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "0"}, "content_type":"text"},
            {"content":{"text": "false"}, "content_type":"text"},
            {"content":{"text": "1"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "0"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_4() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{}},
            {"key":"obj", "value":{"toto": "tutu"}}
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_4");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_5() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{}},
            {"key":"obj", "value":{"_1": "toto"}},
            {"key":"obj", "value":{"_1": "toto", "_2": "toto"}},
            {"key":"obj", "value":{"_1": "toto", "_2": "toto", "_3": "toto"}}
        ],
        "messages":[
            {"content":[], "content_type":"array"},
            {"content":[], "content_type":"array"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_5");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":[], "content_type":"array"},
            {"content":[], "content_type":"array"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_6");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_7() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":[], "content_type":"array"},
            {"content":[], "content_type":"array"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"true"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"},
            {"content":{"text":"toto"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_7");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_8() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text":null}, "content_type":"text"},
            {"content":{"text":null}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(None, "step_8");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
