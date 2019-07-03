use rand::Rng;
use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::parser::{ast::Literal, tokens::*};
use crate::interpreter::message::*;

pub fn remember(name: String, value: String) -> MessageType {
    MessageType::Assign{name, value}
}

pub fn typing<S: BuildHasher>(args: &HashMap<String, Literal, S>, name: String) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Numeric") {
            Some(value) =>  Ok(MessageType::Msg(Message::new(value, name))),
            None        =>  Err("Builtin Typing bad argument type in typing".to_owned())
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn wait<S: BuildHasher>(args: &HashMap<String, Literal, S>, name: String) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Numeric") {
            Some(value) => Ok(MessageType::Msg(Message::new(value, name))),
            None        => Err("Builtin Typing bad argument type in wait".to_owned())
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn text<S: BuildHasher>(args: &HashMap<String, Literal, S>, name: String) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("String") {
            Some(value) => Ok(MessageType::Msg(Message::new(value, name))),
            None        => Err("Builtin Typing bad argument type in text".to_owned())
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn img<S: BuildHasher>(args: &HashMap<String, Literal, S>, name: String) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("String") {
            Some(value) => Ok(MessageType::Msg(Message::new(value, name))),
            None        => Err("Builtin Typing bad argument type in img".to_owned())
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn url<S: BuildHasher>(args: &HashMap<String, Literal, S>, name: String) -> Result<MessageType, String>{
    if args.len() == 1 {
        match args.get("String") {
            Some(value) => Ok(MessageType::Msg(Message::new(value, name))),
            None        => Err("Builtin Typing bad argument type in url".to_owned())
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn one_of<S: BuildHasher>(args: &HashMap<String, Literal, S>, elem_type: String) -> Result<MessageType, String> {
    let lit = &args.values().nth(rand::thread_rng().gen_range(0, args.len())).expect("error in get one_of");
    Ok(MessageType::Msg(Message::new(lit, elem_type)))
}

fn parse_quickbutton(val: Literal, buttton_type: Literal, accepts: &mut Vec<Literal>) -> Literal {
    let mut button_value = HashMap::new();

    accepts.push(val.clone());

    button_value.insert("title".to_owned(), val.clone());
    button_value.insert("buttton_type".to_owned(), Literal::ArrayLiteral(vec![buttton_type]));
    button_value.insert("accept".to_owned(), val.clone());
    button_value.insert("key".to_owned(), val.clone());
    button_value.insert("value".to_owned(), val.clone());
    button_value.insert("payload".to_owned(), val);

    Literal::ObjectLiteral{ name: "button".to_owned(), value: button_value}
}

pub fn question<S: BuildHasher>(args: &HashMap<String, Literal, S>, name: String) -> Result<MessageType, String> {
    let mut question_value = HashMap::new();

    let expr_title = args.get("title").expect("error in question");
    let button_type = args.get("button_type").expect("error in question");
    let expr_buttons = args.get("buttons").expect("error in question");

    let mut accepts: Vec<Literal> = vec![];

    if let Literal::ArrayLiteral(array) = expr_buttons {
        let mut buttons: Vec<Literal> = vec![];

        for button in array.iter() {
            match button {
                Literal::ObjectLiteral{name, value} if name == BUTTON  => {
                    match value.get("String") {
                        Some(elem) => {
                            buttons.push(parse_quickbutton(elem.clone(), button_type.clone(), &mut accepts));
                        },
                        None       => return Err(format!("Builtin Typing bad argument type in question -- {:?} === {:?}", value, value.keys()))
                    }
                },
                err                                                      => return Err(format!("Builtin Typing bad argument type -> {:?}", err ))
            }
        }
        question_value.insert("title".to_owned(), expr_title.clone());
        question_value.insert("accepts".to_owned(), Literal::ArrayLiteral(accepts));
        question_value.insert("buttons".to_owned(), Literal::ArrayLiteral(buttons));
    }

    Ok(MessageType::Msg(
        Message {
            content_type: name.to_lowercase(),
            content: Literal::ObjectLiteral{name: "question".to_owned(), value: question_value}
        }
    ))
}
