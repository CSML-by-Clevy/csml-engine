use serde_json::Value;
use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::error_format::data::ErrorInfo;
use crate::interpreter::builtins::*;
use crate::parser::{ast::Literal, tokens::*};

// default #############################################################################

fn parse_api<S: BuildHasher>(
    args: &HashMap<String, Literal, S>,
) -> Result<(String, HashMap<String, Value>), ErrorInfo> {
    let hostname = match args.get("hostname") {
        Some(Literal::StringLiteral(value)) => value.to_owned(),
        _ => "localhost".to_owned(),
    };
    let port = match args.get("port") {
        Some(Literal::StringLiteral(value)) => value.to_owned(),
        _ => PORT.to_owned(),
    };
    let sub_map = create_submap(&["hostname", "port"], args)?;

    let mut map: HashMap<String, Value> = HashMap::new();

    map.insert("params".to_owned(), Value::Object(sub_map));

    Ok((format!("http://{}:{}/", hostname, port), map))
}

pub fn api<S: BuildHasher>(args: &HashMap<String, Literal, S>) -> Result<Literal, ErrorInfo> {
    let (http_arg, map) = parse_api(&args)?;

    println!("http call {:?}", http_arg);
    println!("map {:?}", serde_json::to_string(&map).unwrap());
    match reqwest::Client::new().post(&http_arg).json(&map).send() {
        Ok(ref mut arg) => match arg.text() {
            Ok(text) => {
                println!("reqwest post ok : ");
                Ok(Literal::StringLiteral(text))
            }
            Err(e) => {
                println!("error in parsing reqwest result: {:?}", e);
                Err("Error in parsing reqwest result".to_owned())
            }
        },
        Err(e) => {
            println!("error in reqwest post {:?}", e);
            Err("Error in reqwest post".to_owned())
        }
    }
}
