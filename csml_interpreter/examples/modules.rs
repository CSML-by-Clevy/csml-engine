use csml_interpreter::data::csml_bot::{CsmlBot, Modules};
use csml_interpreter::data::csml_flow::CsmlFlow;
use csml_interpreter::data::event::Event;
use csml_interpreter::data::Context;
use csml_interpreter::interpret;
use csml_interpreter::load_components;
use csml_interpreter::{search_for_modules, validate_bot};
use std::collections::HashMap;

const DEFAULT_ID_NAME: &str = "id";
const DEFAULT_FLOW_NAME: &str = "default";
const DEFAULT_STEP_NAME: &str = "start";
const DEFAULT_BOT_NAME: &str = "my_bot";

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn main() {
    let default_content = std::fs::read_to_string("CSML/examples/module.csml").unwrap();
    let default_flow = CsmlFlow::new(DEFAULT_ID_NAME, "default", &default_content, Vec::default());

    let native_component = load_components().unwrap();

    // Create a CsmlBot
    let mut bot = CsmlBot::new(
        DEFAULT_ID_NAME,
        DEFAULT_BOT_NAME,
        None,
        vec![default_flow],
        Some(native_component),
        None,
        DEFAULT_FLOW_NAME,
        None,
        None,
        None,
        Some(Modules {
            config: r"- {name: module, url: https://raw.githubusercontent.com/CSML-by-Clevy/csml-engine/dev/csml_engine/CSML/flow2.csml, version: latest }".to_string(),
            flows: vec![],
        }),
    );

    // Create an Event
    let event = Event {
        content_type: "payload".to_owned(), // text
        content_value: "4".to_owned(),
        content: serde_json::json!({"payload":"4"}),
        ttl_duration: None,
        low_data_mode: None,
        secure: false,
    };

    // Create context
    let context = Context::new(
        HashMap::new(),
        HashMap::new(),
        None,
        None,
        DEFAULT_STEP_NAME,
        DEFAULT_FLOW_NAME,
    );

    search_for_modules(&mut bot);

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
