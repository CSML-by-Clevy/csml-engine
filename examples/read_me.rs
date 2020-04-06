use csmlinterpreter::data::csml_flow::CsmlFlow;
use csmlinterpreter::data::csml_bot::CsmlBot;
use csmlinterpreter::interpret;
use csmlinterpreter::data::ContextJson;
use csmlinterpreter::data::event::Event;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn main() {
    let content = std::fs::read_to_string("/Users/jle-quel/Documents/darwin/csml-interpreter/CSML/examples/hello_world.csml").unwrap();

    let flow = vec![CsmlFlow::new("id", "default_flow", &content, Vec::default())];
    let bot = CsmlBot::new("id", "bot", None, flow, "default_flow");
    let event = Event::new("hello");
    let context = ContextJson::default();

    interpret(bot, context, event, None);
}