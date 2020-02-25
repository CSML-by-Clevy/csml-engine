mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, file: &str, step: &str) -> MessageData {
    let text = read_file(format!("CSML/built-in/{}.csml", file)).unwrap();

    let context = gen_context(
        serde_json::json!({}),
        serde_json::json!({}),
    );

    interpret(&text, step, context, &event, None, None, None)
}

#[test]
fn ok_button() {
    let data = r#" {
        "messages":[
            {
                "content": {
                    "accepts": ["hello"],
                    "button_type": "quick_button",
                    "payload": "hello",
                    "title": "hello",

                    "content": {"payload": "hello", "title": "hello"},
                    "content_type": "button"
                },
                "content_type": "button"
            }
        ],
        "next_flow": null,
        "memories": [],
        "next_step": "end"
    }"#;

    let msg = format_message(None, "question", "simple_0");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

// {
//     "messages": Array([
//             Object({
//                 "content": Object({
//                     "buttons": Array([
//                         Object({
//                             "accepts": Array([String("b1")]),
//                             "content": Object({
//                                 "payload": String("b1"),
//                                 "title": String("b1")
//                             }),
//                             "content_type": String("button")
//                         }),
//                         Object({
//                             "accepts": Array([String("b2")]),
//                             "content": Object({
//                                 "payload": String("b2"),
//                                 "title": String("b2")
//                             }),
//                             "content_type": String("button")})
//                     ]),
//                     "title": String("title")
//                 }),
//                 "content_type": String("question")
//             })
//         ]),
//     "next_flow": Null,
//     "memories": Array([]),
//     "next_step": String("end")
// }

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

                        "content": {"payload": "b1", "title": "b1"},
                        "content_type": "button"
                    },
                    {
                        "accepts": ["b2"],
                        "button_type": "quick_button",
                        "payload": "b2",
                        "title": "b2",

                        "content": {"payload": "b2", "title": "b2"},
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
    let msg = format_message(None, "question", "start");

    let v1: Value = message_to_jsonvalue(msg);
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

                        "content": {"payload": "b1", "title": "b1"},
                        "content_type": "button"
                    },
                    {
                        "accepts": ["b2"],
                        "button_type": "quick_button",
                        "payload": "b2",
                        "title": "b2",

                        "content": {"payload": "b2", "title": "b2"},
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
    let msg = format_message(None, "question", "question1");

    let v1: Value = message_to_jsonvalue(msg);
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

                        "content": {"payload": "b1", "title": "b1"},
                        "content_type": "button"
                    },
                    {
                        "accepts": ["b2"],
                        "button_type": "quick_button",
                        "payload": "b2",
                        "title": "b2",

                        "content": {"payload": "b2", "title": "b2"},
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
    let msg = format_message(None, "question", "question2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
