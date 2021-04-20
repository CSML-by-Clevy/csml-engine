mod support;

use csml_interpreter::data::event::Event;
use csml_interpreter::data::hold::{Hold, IndexInfo};
use csml_interpreter::data::Context;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn hold_test_none() {
    let data = r#"
    {"memories":[],
    "messages":[
        {"content":{"error":"< this_hold > is used before it was saved in memory at line 2, column 5 at flow [flow]"}, "content_type":"error"},
        {"content":{"text":"1"}, "content_type":"text"},
        {"content":{"text":"2"}, "content_type":"text"},
        {"content":{"error": "< this_hold > is used before it was saved in memory at line 8, column 6 at flow [flow]"}, "content_type":"error"},
        {"content":{"text":"4"}, "content_type":"text"}]
    }
    "#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "start", "flow"),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_0() {
    let data = r#"
    {
        "memories":[],
        "messages":[
            {"content":{"text":"1"}, "content_type":"text"},
            {"content":{"text":"2"}, "content_type":"text"},
            {"content":{"error": "< this_hold > is used before it was saved in memory at line 8, column 6 at flow [flow]"}, "content_type":"error"},
            {"content":{"text":"4"}, "content_type":"text"}
        ]
    }
    "#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 0,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_3() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"4"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 3,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_7() {
    let data = r#"{"memories":[], "messages":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 7,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_8() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"3"}, "content_type":"text"}]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 8,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_some_17() {
    let data = r#"{"memories":[], "messages":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 17,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "start",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}


#[test]
fn hold_test_step_1_ok() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"OK"}, "content_type":"text"}] }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 4,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "hold_1_ok",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_step_2_ok() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"OK"}, "content_type":"text"}] }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 5,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "hold_2_ok",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_step_3_ok() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"OK"}, "content_type":"text"}] }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 6,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "hold_3_ok",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn hold_test_step_4_ok() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"OK"}, "content_type":"text"}] }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            Some(Hold::new(
                IndexInfo {
                    command_index: 4,
                    loop_index: vec![],
                },
                serde_json::json!({}),
                "".to_owned(),
                "".to_owned(),
            )),
            "hold_4_ok",
            "flow",
        ),
        "CSML/basic_test/hold.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}