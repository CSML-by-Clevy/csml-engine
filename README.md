# CSML Language

![CSML logo](./images/csml-horizontal-whitebg-v3.png)

## Introduction

The CSML (Conversational Standard Meta Language) is a Domain-Specific Language developed for creating conversational experiences easily.

The purpose of this language is to simplify the creation and maintenance of rich conversational interactions between humans and machines. With a very expressive and text-only syntax, CSML flows are easy to understand, making it easy to deploy and maintain conversational agents. The CSML handles short and long-term memory slots, metadata injection, and connecting to any third party API or injecting arbitrary code in any programming language thanks to its powerful runtime APIs.

By using the CSML language, any developer can integrate arbitrarily complex conversational agents on any channel (Facebook Messenger, Slack, Facebook Workplace, Microsoft Teams, custom webapp, ...) and make any bot available to any end user. The CSML platform comes with a large number of channel integrations that work out of the box, but developers are free to add new custom integrations by using the CSML interfaces.

## Functional diagram

![diagram](./images/csml-interpreter.png)

## Examples

### Hello World

    cargo run --example hello_world

### Event

    cargo run --example event

### Metadata

    cargo run --example metadata

### Memory

    cargo run --example memory

## Quick Start run it yourself

 requires Rust version 1.41.

```rust
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
    let content = std::fs::read_to_string("./hello_world.csml").unwrap();

    // Create a CsmlFlow
    let flow = CsmlFlow::new(
        DEFAULT_ID_NAME,
        DEFAULT_FLOW_NAME,
        &content,
        Vec::default()
    );

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

    // Create a Context
    let context = ContextJson::new(
        serde_json::json!({}),
        serde_json::json!({}),
        None,
        None,
        DEFAULT_STEP_NAME,
        DEFAULT_FLOW_NAME,
    );

    // Run interpreter
    dbg!(interpret(bot, context, event, None));
}
```

## Additional Information

### Getting Help

* [Slack] - Direct questions about using the language.
* [CSML Documentation](https://docs.csml.dev) - Getting started.

[Slack]: https://csml-by-clevy.slack.com/join/shared_invite/enQtODAxMzY2MDQ4Mjk0LWZjOTZlODI0YTMxZTg4ZGIwZDEzYTRlYmU1NmZjYWM2MjAwZTU5MmU2NDdhNmU2N2Q5ZTU2ZTcxZDYzNTBhNTc

### Information

* [Roadmap](https://trello.com/b/tZ1MoALL/csml-open-roadmap) - Upcoming new features.
* [Release notes](https://headwayapp.co/csml-release-notes) - Stay up to date.

### Play with the language

* [Studio] - Create and deploy your chatbot in a matter of minutes.

[Studio]: https://studio.csml.dev
