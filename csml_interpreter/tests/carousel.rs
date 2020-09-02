mod support;

use csml_interpreter::data::context::ContextJson;
use csml_interpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

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
                            "content": {
                                "accepts": ["b1", "b1"],
                                "payload": "b1",
                                "title": "b1",
                                "theme": "primary",
                                "icon": "info"
                            },
                            "content_type": "button"
                        }
                    ]
                },
                "content_type": "card"
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
        "CSML/basic_test/built-in/carousel.csml",
    );

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
                        "content": {
                            "title": "c1",
                            "buttons": [
                                {
                                    "content": {
                                        "accepts": ["b1", "b1"],
                                        "payload": "b1",
                                        "title": "b1"
                                    },
                                    "content_type": "button"
                                }
                            ]
                        },
                        "content_type": "card"
                    }
                ]
            },
            "content_type": "carousel"
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
        "CSML/basic_test/built-in/carousel.csml",
    );

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
                        "content": {
                            "title": "c1",
                            "buttons": [
                                {
                                    "content": {
                                        "accepts": ["b1", "b1"],
                                        "payload": "b1",
                                        "title": "b1",
                                        "theme": "primary",
                                        "icon": "info"
                                    },
                                    "content_type": "button"
                                }
                            ]
                        },
                        "content_type": "card"
                    }
                ]
            },
            "content_type": "carousel"
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
            "carousel1",
            "flow",
        ),
        "CSML/basic_test/built-in/carousel.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
