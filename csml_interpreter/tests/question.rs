mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_button() {
    let data = r#" {
        "messages":[
            {
                "content": {
                    "accepts": ["toto", "hello", "test"],
                    "button_type": "quick_button",
                    "payload": "test",
                    "title": "hello"
                },
                "content_type": "button"
            }
        ],
        "memories": []
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "simple_0",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/question.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_question() {
    let data = r#"
    {"messages":
        [ {
            "content": {
                "buttons": [
                    {
                        "content": {
                            "accepts": ["b1", "b1"],
                            "payload": "b1",
                            "title": "b1"
                        },
                        "content_type": "button"
                    },
                    {
                        "content": {
                            "accepts": ["b2", "b2"],
                            "payload": "b2",
                            "title": "b2"
                        },
                        "content_type": "button"
                    }
                ],
                "title": "title"
            },
            "content_type":"question"
        } ],
    "memories":[]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "start",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/question.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_question_step1() {
    let data = r#"
    { "messages":
        [ {
            "content": {
                "buttons": [
                    {
                        "content": {
                            "accepts": ["b1", "b1"],
                            "payload": "b1",
                            "title": "b1"
                        },

                        "content_type": "button"
                    },
                    {
                        "content": {
                            "accepts": ["b2", "b2"],
                            "payload": "b2",
                            "title": "b2"
                        },

                        "content_type": "button"
                    }
                ],
                "title": "title"
            },
            "content_type":"question"
        } ],
    "memories":[]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "question1",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/question.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_question_step2() {
    let data = r#"
    { "messages":
        [ {
            "content": {
                "buttons": [
                    {
                        "content": {
                            "accepts": ["b1", "b1"],
                            "payload": "b1",
                            "title": "b1"
                        },

                        "content_type": "button"
                    },
                    {
                        "content": {
                            "accepts": ["b2", "b2"],
                            "payload": "b2",
                            "title": "b2"
                        },

                        "content_type": "button"
                    }
                ]
            },
            "content_type":"question"
        } ],
    "memories":[]
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "question2",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/question.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
