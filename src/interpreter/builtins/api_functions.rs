use std::io::{Error, ErrorKind, Result};
use serde_json::{Value};
use std::collections::HashMap;

use crate::parser::{ast::{Expr, Literal}, tokens::*};
use crate::interpreter:: {
    message::*,
    json_to_rust::*,
    builtins::*,
};

// default #############################################################################

fn parse_api(vec: &[Expr], memory: &Memory, event: &Option<Event>) -> Result<(String, HashMap<String, Value>) > {
    let hostname = value_or_default("hostname", vec, Some("localhost".to_owned()), memory, event)?;
    let port = value_or_default("port", vec, Some(PORT.to_owned()), memory, event)?;
    let sub_map = create_submap(&["hostname", "port"], vec, memory, event)?;

    let mut map: HashMap<String, Value> = HashMap::new();

    map.insert("params".to_owned(), Value::Object(sub_map));

    Ok((format!("http://{}:{}/", hostname, port), map))
}

pub fn api(args: &Expr, memory: &Memory, event: &Option<Event>) -> Result<MessageType> {
    if let Expr::VecExpr(vec) = args {
        let (http_arg, map) = parse_api(&vec, memory, event)?;

        println!("http call {:?}", http_arg);
        println!("map {:?}", serde_json::to_string(&map).unwrap());
        match reqwest::Client::new().post(&http_arg).json(&map).send() {
            Ok(ref mut arg) => {
                match arg.text() {
                    Ok(text) => {
                        println!("reqwest post ok : ");
                        return Ok(MessageType::Msg(Message::new( &Expr::LitExpr{lit: Literal::StringLiteral(text)} , "text".to_owned())))
                    },
                    Err(e)  => {
                        println!("error in parsing reqwest result: {:?}", e);
                        return Err(Error::new(ErrorKind::Other, "Error in parsing reqwest result"))
                    }
                }
            },
            Err(e) => {
                println!("error in reqwest post {:?}", e);
                return Err(Error::new(ErrorKind::Other, "Error in reqwest post"))
            }
        };
    }

    println!("default is not correctly formatted");
    Err(Error::new(ErrorKind::Other, "Builtin default bad argument"))
}

