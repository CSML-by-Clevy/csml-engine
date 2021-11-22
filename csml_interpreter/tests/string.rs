mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn string_step_0() {
    let data = r#"{
        "memories":[
            {"key":"s", "value":"Hello "},
            {"key":"s", "value":"Hello World"}
        ],
        "messages":[
            {"content":{"text": "Hello World"}, "content_type":"text"},
            {"content":{"text": "HELLO WORLD"}, "content_type":"text"},
            {"content":{"text": "hello world"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_0", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_1() {
    let data = r#"{
        "memories":[
            {"key":"s", "value":"Hello"}
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_1", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_2() {
    let data = r#"{
        "memories":[
            {"key":"s", "value":"Hello"}
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "true"}, "content_type":"text"},
            {"content":{"text": "false"}, "content_type":"text"},
            {"content":{"text": "false"}, "content_type":"text"},
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_2", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_3() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_3", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_4() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "[\"Hello\"]"}},
            {"content_type":"text", "content":{"text": "[\"\",\"ello\"]"}},
            {"content_type":"text", "content":{"text": "[\"He\",\"\",\"o\"]"}},
            {"content_type":"text", "content":{"text": "[\"He\",\"o\"]"}},
            {"content_type":"text", "content":{"text": "[\"Hell\",\"\"]"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_4", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_5() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "Hello World"}},
            {"content_type":"text", "content":{"text": "Hello"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_5", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_6() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "Hello World"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_6", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_7() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "😆"}},
            {"content_type":"text", "content":{"text": "H"}},
            {"content_type":"text", "content":{"text": "e"}},
            {"content_type":"text", "content":{"text": "l"}},
            {"content_type":"text", "content":{"text": "l"}},
            {"content_type":"text", "content":{"text": "o"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_7", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_8() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "😆"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_8", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_9() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "😆"}},
            {"content_type":"text", "content":{"text": "😆Hello World 😆"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "step_9", "flow"),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_10() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "test Hello World 😆"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_10",
            "flow",
        ),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_11() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "false"}},
            {"content_type":"text", "content":{"text": "Hello World"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_11",
            "flow",
        ),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}


#[test]
fn string_step_12_slice() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "Hel"}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_12_slice",
            "flow",
        ),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_13_to_string() {
    let data = r#"{
        "memories":[],
        "messages":[
            {"content_type":"text", "content":{"text": "\"4\""}}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "step_13_to_string",
            "flow",
        ),
        "CSML/basic_test/stdlib/string.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}