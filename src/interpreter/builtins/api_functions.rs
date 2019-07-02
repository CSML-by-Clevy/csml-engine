use serde_json::{Value};
use std::collections::HashMap;

use crate::parser::{ast::{Expr, Literal}, tokens::*};
use crate::interpreter:: {
    message::*,
    builtins::*,
    data::Data
};

// default #############################################################################

fn parse_api(args: &HashMap<String, Literal>, data: &mut Data) -> Result<(String, HashMap<String, Value>), String> {
    let hostname = match args.get("hostname") { Some(Literal::StringLiteral(value)) => value.to_owned(), _ => "localhost".to_owned() };
    let port = match args.get("port") { Some(Literal::StringLiteral(value)) => value.to_owned(), _ => PORT.to_owned() };
    let sub_map = create_submap(&["hostname", "port"], args, data)?;

    let mut map: HashMap<String, Value> = HashMap::new();

    map.insert("params".to_owned(), Value::Object(sub_map));

    Ok((format!("http://{}:{}/", hostname, port), map))
}

// 
// let mut question_value = HashMap::new();

//     let expr_title = args.get("title").expect("error in question");
//     let button_type = args.get("button_type").expect("error in question");
//     let expr_buttons = args.get("buttons").expect("error in question");

//     let mut accepts: Vec<Literal> = vec![];

//     if let Literal::ArrayLiteral(array) = expr_buttons {
//         let buttons: Vec<Literal> = vec![];

//         for button in array.iter() {
//             match button {
//                 Literal::ObjectLiteral{name, value} if name == BUTTON  => {
//                     match value.get("String") {
//                         Some(elem) => {
//                             buttons.push(parse_quickbutton(elem.clone(), button_type.clone(), &mut accepts));
//                         },
//                         None       => return Err("Builtin Typing bad argument type".to_owned())
//                     }
//                 },
//                 _                                                      => return Err("Builtin Typing bad argument type".to_owned())
//             }
//         }
//         question_value.insert("title".to_owned(), expr_title.clone());
//         question_value.insert("accepts".to_owned(), Literal::ArrayLiteral(accepts));
//         question_value.insert("buttons".to_owned(), Literal::ArrayLiteral(buttons));
//     }

//     Ok(MessageType::Msg(
//         Message {
//             content_type: name.to_lowercase(),
//             content: Literal::ObjectLiteral{name: "question".to_owned(), value: question_value}
//         }
//     ))
// 

pub fn api(args: &HashMap<String, Literal>, data: &mut Data) -> Result<MessageType, String> {
    let (http_arg, map) = parse_api(&args, data)?;

    println!("http call {:?}", http_arg);
    println!("map {:?}", serde_json::to_string(&map).unwrap());
    match reqwest::Client::new().post(&http_arg).json(&map).send() {
        Ok(ref mut arg) => {
            match arg.text() {
                Ok(text) => {
                    println!("reqwest post ok : ");
                    return Ok(MessageType::Msg(Message::new( &Literal::StringLiteral(text), "text".to_owned())))
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
