use csmlinterpreter::data::csml_bot::CsmlBot;
use csmlinterpreter::data::csml_flow::CsmlFlow;
use csmlinterpreter::data::event::Event;
use csmlinterpreter::data::ContextJson;
use csmlinterpreter::interpret;

const DEFAULT_ID_NAME: &str = "id";
const DEFAULT_FLOW_NAME: &str = "flow";
const DEFAULT_STEP_NAME: &str = "start";
const DEFAULT_BOT_NAME: &str = "my_bot";

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn main() {
    let content = std::fs::read_to_string(
        "/Users/jle-quel/Documents/csml/interpreter/CSML/examples/hello_world.csml",
    )
    .unwrap();

    // Create a CsmlFlow
    let flow = CsmlFlow::new(DEFAULT_ID_NAME, DEFAULT_FLOW_NAME, &content, Vec::default());

    // Create a CsmlBot
    let bot = CsmlBot::new(
        DEFAULT_ID_NAME,
        DEFAULT_BOT_NAME,
        None,
        vec![flow],
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
    interpret(bot, context, event, None);
}
