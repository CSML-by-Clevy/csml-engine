use csmlinterpreter::data::csml_bot::CsmlBot;
use csmlinterpreter::data::csml_flow::CsmlFlow;
use csmlinterpreter::data::event::Event;
use csmlinterpreter::data::ContextJson;
use csmlinterpreter::validate_bot;
use csmlinterpreter::{interpret, load_components};

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
    );

    // Create an Event
    let event = Event::default();

    // Create context
    let context = ContextJson::new(
        serde_json::json!({}),
        serde_json::json!({}),
        None,
        None,
        DEFAULT_STEP_NAME,
        DEFAULT_FLOW_NAME,
    );

    // Run interpreter
    let result = validate_bot(bot.to_owned());

    if result.errors.is_some() {
        dbg!(result.errors);
        return;
    }
    if result.warnings.is_some() {
        dbg!(result.warnings);
    }

    dbg!(interpret(bot, context, event, None));
}
