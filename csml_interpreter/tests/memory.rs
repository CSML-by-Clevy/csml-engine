mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::csml_bot::CsmlBot;
use csml_interpreter::data::csml_flow::CsmlFlow;
use csml_interpreter::data::event::Event;
use csml_interpreter::data::MessageData;
use csml_interpreter::interpret;
use std::collections::HashMap;

use crate::support::tools::message_to_json_value;
use crate::support::tools::read_file;

use serde_json::Value;

const DEFAULT_ID_NAME: &str = "id";
const DEFAULT_FLOW_NAME: &str = "default";
const DEFAULT_STEP_NAME: &str = "start";
const DEFAULT_BOT_NAME: &str = "my_bot";

fn format_message(event: Event, context: Context, vector: &[&str]) -> MessageData {
    let default_content = read_file(vector[0].to_string()).unwrap();
    let default_flow = CsmlFlow::new(DEFAULT_ID_NAME, "default", &default_content, Vec::default());

    let other_content = std::fs::read_to_string(vector[1].to_string()).unwrap();
    let other_flow = CsmlFlow::new(DEFAULT_ID_NAME, "other", &other_content, Vec::default());

    let bot = CsmlBot::new(
        DEFAULT_ID_NAME,
        DEFAULT_BOT_NAME,
        None,
        vec![default_flow, other_flow],
        None,
        None,
        DEFAULT_FLOW_NAME,
        None,
        None,
        None,
    );

    interpret(bot, context, event, None)
}

#[test]
fn memory() {
    let data = r#"
        {"memories":[{"key":"var", "value":42}], "messages":[
        {
            "content": {"text": "var from start: 42"},
            "content_type": "text"
        },
        {
            "content": {"text": "var from step: 42"},
            "content_type": "text"
        },
        {
            "content": {"text": "var from flow: 42"},
            "content_type": "text"
        }
        ]}"#;

    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            DEFAULT_STEP_NAME,
            DEFAULT_FLOW_NAME,
        ),
        &vec![
            "CSML/basic_test/bot/default.csml",
            "CSML/basic_test/bot/other.csml",
        ],
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
