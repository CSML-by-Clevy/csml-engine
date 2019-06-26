use serde_json::{Value};
use std::collections::HashMap;

use crate::parser::{ast::{Expr, Literal}, tokens::*};
use crate::interpreter:: {
    message::*,
    builtins::*,
    data::Data
};

// default #############################################################################

fn parse_api(vec: &[Expr], data: &mut Data) -> Result<(String, HashMap<String, Value>), String> {
    let hostname = value_or_default("hostname", vec, Some("localhost".to_owned()), data)?;
    let port = value_or_default("port", vec, Some(PORT.to_owned()), data)?;
    let sub_map = create_submap(&["hostname", "port"], vec, data)?;

    let mut map: HashMap<String, Value> = HashMap::new();

    map.insert("params".to_owned(), Value::Object(sub_map));

    Ok((format!("http://{}:{}/", hostname, port), map))
}

pub fn api(args: &Expr, data: &mut Data) -> Result<MessageType, String> {
    if let Expr::VecExpr(vec) = args {
        let (http_arg, map) = parse_api(&vec, data)?;

        println!("http call {:?}", http_arg);
        println!("map {:?}", serde_json::to_string(&map).unwrap());
        match reqwest::Client::new().post(&http_arg).json(&map).send() {
            Ok(ref mut arg) => {
                match arg.text() {
                    Ok(text) => {
                        println!("reqwest post ok : ");
                        return Ok(MessageType::Msg(Message::new( &Expr::LitExpr{lit: Literal::StringLiteral(text)}, "text".to_owned())))
                    },
                    Err(e)  => {
                        println!("error in parsing reqwest result: {:?}", e);
                        return Err("Error in parsing reqwest result".to_owned())
                    }
                }
            },
            Err(e) => {
                println!("error in reqwest post {:?}", e);
                return Err("Error in reqwest post".to_owned())
            }
        };
    }

    println!("default is not correctly formatted");
    Err("Builtin default bad argument".to_owned())
}
