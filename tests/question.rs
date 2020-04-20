mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

fn format_message(event: Event, file: &str, step: &str) -> MessageData {
    let text = read_file(format!("CSML/basic_test/built-in/{}.csml", file)).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn ok_button() {
    let data = r#" {
        "messages":[
            {
                "content": {
                    "accepts": ["toto", "hello"],
                    "button_type": "quick_button",
                    "payload": "hello",
                    "title": "hello"
                },
                "content_type": "button"
            }
        ],
        "next_flow": null,
        "memories": [],
        "next_step": "end"
    }"#;

    let msg = format_message(gen_event(""), "question", "simple_0");

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
                        "content": {
                            "accepts": ["b1"],
                            "payload": "b1",
                            "title": "b1"
                        },
                        "content_type": "button"
                    },
                    {
                        "content": {
                            "accepts": ["b2"],
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
    "next_flow":null,
    "memories":[],
    "next_step":"end"
    }"#;
    let msg = format_message(gen_event(""), "question", "start");

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
                        "content": {
                            "accepts": ["b1"],
                            "payload": "b1",
                            "title": "b1"
                        },

                        "content_type": "button"
                    },
                    {
                        "content": {
                            "accepts": ["b2"],
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
    "next_flow":null,
    "memories":[],
    "next_step":"end"
    }"#;
    let msg = format_message(gen_event(""), "question", "question1");

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
                        "content": {
                            "accepts": ["b1"],
                            "payload": "b1",
                            "title": "b1"
                        },

                        "content_type": "button"
                    },
                    {
                        "content": {
                            "accepts": ["b2"],
                            "payload": "b2",
                            "title": "b2"
                        },

                        "content_type": "button"
                    }
                ]
            },
            "content_type":"question"
        } ],
    "next_flow":null,
    "memories":[],
    "next_step":"end"
    }"#;
    let msg = format_message(gen_event(""), "question", "question2");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
