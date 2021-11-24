use csml_interpreter::data::csml_bot::CsmlBot;
use csml_interpreter::data::csml_flow::CsmlFlow;
use csml_interpreter::data::event::Event;
use csml_interpreter::data::Context;
use csml_interpreter::validate_bot;
use csml_interpreter::{interpret, load_components};
use std::collections::HashMap;

const DEFAULT_ID_NAME: &str = "id";
const DEFAULT_FLOW_NAME: &str = "default";
const DEFAULT_STEP_NAME: &str = "start";
const DEFAULT_BOT_NAME: &str = "my_bot";

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn main() {
    let default_content = std::fs::read_to_string("CSML/examples/bot/default.csml").unwrap();
    let default_flow = CsmlFlow::new(DEFAULT_ID_NAME, "default", &default_content, Vec::default());

    let native_components = load_components().unwrap();
    let mut custom_components = serde_json::Map::new();

    custom_components.insert(
        "Button".to_owned(),
        serde_json::json!({
          "params": [
            {
              "title": {
                "required": true,
                "type": "String",
                "default_value": [],
                "add_value": []
              }
            },
            {
              "payload": {
                "type": "String",
                "default_value": [
                  {
                    "$_get": "title"
                  }
                ],
                "add_value": []
              }
            },
            {
              "accepts": {
                "type": "Array",
                "default_value": [
                  {
                    "$_get": "title"
                  }
                ],
                "add_value": []
              }
            }
          ]
        }),
    );
    custom_components.insert(
        "Question".to_owned(),
        serde_json::json!({
            "params": [
                {
                    "title": {
                        "required": true,
                        "type": "String",
                        "default_value": [],
                        "add_value": []
                    }
                },
                {
                    "buttons": {
                        "required": true,
                        "type": "Array",
                        "default_value": [],
                        "add_value": []
                    }
                },
            ]
        }),
    );

    // Create a CsmlBot
    let bot = CsmlBot::new(
        DEFAULT_ID_NAME,
        DEFAULT_BOT_NAME,
        None,
        vec![default_flow],
        Some(native_components),
        Some(serde_json::json!(custom_components)),
        DEFAULT_FLOW_NAME,
        None,
        None,
        None,
    );

    // Create an Event
    let event = Event::default();

    // Create context
    let context = Context::new(
        HashMap::new(),
        HashMap::new(),
        None,
        None,
        DEFAULT_STEP_NAME,
        DEFAULT_FLOW_NAME,
    );

    // Run interpreter
    let result = validate_bot(&bot);

    if result.errors.is_some() {
        dbg!(result.errors);
        return;
    }
    if result.warnings.is_some() {
        dbg!(result.warnings);
    }

    dbg!(interpret(bot, context, event, None));
}
