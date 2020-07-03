mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn object_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    assert_eq!(msg.messages[0].content_type, "error")
}

#[test]
fn object_step_1() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{"_content":{}, "_content_type":"object"} },
            {"key":"obj", "value": {"_content":{"42": "Hello"}, "_content_type":"object"} }
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_2() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{"_content":{}, "_content_type":"object"}},
            {"key":"obj", "value":{"_content":{}, "_content_type":"object"}},
            {"key":"obj", "value": {"_content":{"Hello": 42}, "_content_type":"object"} },
            {"key":"obj", "value":{"_content":{}, "_content_type":"object"}}
        ],
        "messages":[
            {"content":{"text": null}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_2",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_3() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value": {"_content":{}, "_content_type":"object"}},
            {"key":"obj", "value": {"_content":{"42": "Hello"}, "_content_type":"object"}},
            {"key":"obj", "value": {"_content":{}, "_content_type":"object"}}
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "0"}, "content_type":"text"},
            {"content":{"text": "false"}, "content_type":"text"},
            {"content":{"text": "1"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "0"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_3",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_4() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{"_content":{}, "_content_type":"object"}},
            {"key":"obj", "value":{"_content":{"toto": "tutu"}, "_content_type":"object"}}
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_4",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn object_step_5() {
    let data = r#"{
        "memories":[
            {"key":"obj", "value":{"_content":{}, "_content_type":"object"}},
            {"key":"obj", "value":{"_content":{"_1": "toto"}, "_content_type":"object"} },
            {"key":"obj", "value":{"_content":{"_1": "toto", "_2": "toto"}, "_content_type":"object"} },
            {"key":"obj", "value":{"_content":{"_1": "toto", "_2": "toto", "_3": "toto"}, "_content_type":"object"} }
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
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_5",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    let v1: Value = message_to_json_value(msg);
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
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_6",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    let v1: Value = message_to_json_value(msg);
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
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_7",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    let v1: Value = message_to_json_value(msg);
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
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "step_8",
            "flow",
        ),
        "CSML/basic_test/stdlib/object.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
