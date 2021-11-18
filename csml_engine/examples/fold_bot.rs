use csml_engine::{fold_bot};
use csml_interpreter::{
    data::{csml_bot::CsmlBot, csml_flow::CsmlFlow},
    load_components,
};
use std::fs::File;
use std::io::prelude::*;

fn get_flow(name: &str) -> Result<String, ::std::io::Error> {
    let file_path = format!("CSML/{}.csml", name);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn init_bot() -> CsmlBot {
    CsmlBot {
        id: "botid".to_owned(),
        name: "plop".to_owned(),
        fn_endpoint: Some("endpoint".to_owned()),
        flows: vec![
            CsmlFlow {
                id: "flowid".to_owned(),
                name: "flow".to_owned(),
                content: get_flow("flow").expect("error in reading flow"),
                commands: vec!["/plop".to_owned()],
            },
            CsmlFlow {
                id: "2".to_owned(),
                name: "flow2".to_owned(),
                content: get_flow("flow2").expect("error in reading flow"),
                commands: vec!["/random".to_owned()],
            },
        ],
        native_components: Some(load_components().unwrap()),
        custom_components: None,
        default_flow: "flow".to_owned(),
        bot_ast: None,
        no_interruption_delay: None,
        env: None,
    }
}

fn main() {
    let bot = init_bot();

    let folded = fold_bot(bot).unwrap();

    let path = "bot_folded2.txt";

    let mut output = File::create(path).unwrap();

    write!(output, "{}", &folded).unwrap();
}
