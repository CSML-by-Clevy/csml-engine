mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_button() {
    let data = r#" {
        "messages":[
            {
                "content": {
                    "accepts": ["toto", "hello"],
                    "button_type": "quick_button",
                    "payload": "hello",
                    "title": "hello",
                    "theme": "primary",

                    "content": {"payload": "hello", "title": "hello"},
                    "content_type": "button"
                },
                "content_type": "button"
            }
        ],
        "memories": []
    }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "simple_0",
            "flow",
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
                "accepts": ["b1", "b2"],
                "buttons": [
                    {
                        "accepts": ["b1"],
                        "button_type": "quick_button",
                        "payload": "b1",
                        "title": "b1",
                        "theme": "primary",

                        "content": {"payload": "b1", "title": "b1"},
                        "content_type": "button"
                    },
                    {
                        "accepts": ["b2"],
                        "button_type": "quick_button",
                        "payload": "b2",
                        "title": "b2",
                        "theme": "primary",

                        "content": {"payload": "b2", "title": "b2"},
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
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "start",
            "flow",
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
                "accepts": ["b1", "b2"],
                "buttons": [
                    {
                        "accepts": ["b1"],
                        "button_type": "quick_button",
                        "payload": "b1",
                        "title": "b1",
                        "theme": "primary",

                        "content": {"payload": "b1", "title": "b1"},
                        "content_type": "button"
                    },
                    {
                        "accepts": ["b2"],
                        "button_type": "quick_button",
                        "payload": "b2",
                        "title": "b2",
                        "theme": "primary",

                        "content": {"payload": "b2", "title": "b2"},
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
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "question1",
            "flow",
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
                "accepts": ["b1", "b2"],
                "buttons": [
                    {
                        "accepts": ["b1"],
                        "button_type": "quick_button",
                        "payload": "b1",
                        "title": "b1",
                        "theme": "primary",

                        "content": {"payload": "b1", "title": "b1"},
                        "content_type": "button"
                    },
                    {
                        "accepts": ["b2"],
                        "button_type": "quick_button",
                        "payload": "b2",
                        "title": "b2",
                        "theme": "primary",

                        "content": {"payload": "b2", "title": "b2"},
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
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "question2",
            "flow",
        ),
        "CSML/basic_test/built-in/question.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
