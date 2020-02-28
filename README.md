# CSML Language
![](./images/csml-horizontal-whitebg-v3.png)

## Introduction
The CSML (Conversational Standard Meta Language) is a Domain-Specific Language developed for creating conversational experiences easily.

The purpose of this language is to simplify the creation and maintenance of rich conversational interactions between humans and machines. With a very expressive and text-only syntax, CSML flows are easy to understand, making it easy to deploy and maintain conversational agents. The CSML handles short and long-term memory slots, metadata injection, and connecting to any third party API or injecting arbitrary code in any programming language thanks to its powerful runtime APIs.

By using the CSML language, any developer can integrate arbitrarily complex conversational agents on any channel (Facebook Messenger, Slack, Facebook Workplace, Microsoft Teams, custom webapp, ...) and make any bot available to any end user. The CSML platform comes with a large number of channel integrations that work out of the box, but developers are free to add new custom integrations by using the CSML interfaces.

## Examples

###### Hello World
    cargo run --example hello_world

###### Event
    cargo run --example event

###### Metadata
    cargo run --example metadata

###### Memory
    cargo run --example memory

# Quick Start run it yourself
 requires Rust version 1.41.

```rust
use std::fs::File;
use std::io::prelude::*;
use csmlinterpreter::{
    data::ContextJson,
    interpret,
};

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn interpret_flow(flow: &str, step_name: &str) {
    let event = Event::text("hello");
    let context = ContextJson::new();

    dbg!(interpret(
        flow, step_name, context, &event, None
    ));
}

fn main() {
    //add a valid path to a csml file
    let flow = read_file("./hello_world.csml".to_owned()).unwrap();

    interpret_flow(&flow, "start");
}
```

## Links
    - [Playground]: https://studio.csml.dev
    - [Roadmap]: https://trello.com/b/tZ1MoALL/csml-open-roadmap
    - [Release notes]: https://headwayapp.co/csml-release-notes
## Documentations
    - [CSML Documentation]: https://docs.csml.dev
<!-- lib rs docs -->
