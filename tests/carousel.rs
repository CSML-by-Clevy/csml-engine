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
fn ok_card() {
    let data = r#" {
        "messages":[
            {
                "content": {
                    "title": "c1",
                    "image_url": "url",
                    "buttons": [
                        {
                                "accepts": ["b1"],
                                "button_type": "quick_button",
                                "payload": "b1",
                                "title": "b1",
                                "theme": "primary",
                                "content_type": "button"
                        }
                    ]
                },
                "content_type": "card"
            }
        ],
        "next_flow": null,
        "memories": [],
        "next_step": "end"
    }"#;

    let msg = format_message(gen_event(""), "carousel", "simple_0");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_carousel() {
    let data = r#"
    {"messages":
        [ {
            "content": {
                "cards": [
                    {
                        "title": "c1",
                        "content_type": "card",
                        "buttons": [
                            {
                                    "accepts": ["b1"],
                                    "button_type": "quick_button",
                                    "payload": "b1",
                                    "title": "b1",
                                    "theme": "primary",
                                    "content_type": "button"
                            }
                        ]
                    }
                ]
            },
            "content_type": "carousel"
        } ],
    "next_flow":null,
    "memories":[],
    "next_step":"end"
    }"#;
    let msg = format_message(gen_event(""), "carousel", "start");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
#[test]
fn ok_carousel_step1() {
    let data = r#"
    {"messages":
        [ {
            "content": {
                "cards": [
                    {
                        "title": "c1",
                        "content_type": "card",
                        "buttons": [
                            {
                                    "accepts": ["b1"],
                                    "button_type": "quick_button",
                                    "payload": "b1",
                                    "title": "b1",
                                    "theme": "primary",
                                    "icon": "info",
                                    "content_type": "button"
                            }
                        ]
                    }
                ]
            },
            "content_type": "carousel"
        } ],
    "next_flow":null,
    "memories":[],
    "next_step":"end"
    }"#;
    let msg = format_message(gen_event(""), "carousel", "carousel1");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
