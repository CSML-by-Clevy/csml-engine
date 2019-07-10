use rand::Rng;
use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::interpreter::message::*;
use crate::parser::{ast::Literal, tokens::*};

pub fn typing<S: BuildHasher>(args: &HashMap<String, Literal, S>) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Numeric") {
            Some(value) => Ok(MessageType::Msg(Message::new(value))),
            None => Err("Builtin Typing bad argument type in typing".to_owned()),
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn wait<S: BuildHasher>(args: &HashMap<String, Literal, S>) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Numeric") {
            Some(value) => Ok(MessageType::Msg(Message::new(value))),
            None => Err("Builtin Typing bad argument type in wait".to_owned()),
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn text<S: BuildHasher>(args: &HashMap<String, Literal, S>) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Text") {
            Some(value) => Ok(MessageType::Msg(Message::new(value))),
            None => Err("Builtin Typing bad argument type in text".to_owned()),
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn img<S: BuildHasher>(args: &HashMap<String, Literal, S>) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Text") {
            Some(value) => Ok(MessageType::Msg(Message::new(value))),
            None => Err("Builtin Typing bad argument type in img".to_owned()),
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn url<S: BuildHasher>(args: &HashMap<String, Literal, S>) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Text") {
            Some(value) => Ok(MessageType::Msg(Message::new(value))),
            None => Err("Builtin Typing bad argument type in url".to_owned()),
        }
    } else {
        Err("Builtin Typing bad number of argument".to_owned())
    }
}

pub fn one_of<S: BuildHasher>(args: &HashMap<String, Literal, S>) -> Result<MessageType, String> {
    let lit = &args
        .values()
        .nth(rand::thread_rng().gen_range(0, args.len()))
        .expect("error in get one_of");
    Ok(MessageType::Msg(Message::new(lit)))
}

fn parse_quickbutton(val: Literal, buttton_type: Literal) -> Literal {
    let mut button_value = HashMap::new();

    button_value.insert("title".to_owned(), val.clone());
    button_value.insert(
        "buttton_type".to_owned(),
        Literal::ArrayLiteral(vec![buttton_type]),
    );
    button_value.insert("accept".to_owned(), val.clone());
    button_value.insert("key".to_owned(), val.clone());
    button_value.insert("value".to_owned(), val.clone());
    button_value.insert("payload".to_owned(), val);

    Literal::ObjectLiteral {
        name: "button".to_owned(),
        value: button_value,
    }
}

fn get_one_of<S: BuildHasher>(map: &HashMap<String, Literal, S>) -> Result<Literal, String> {
    if let Some(elem) = map.get("Text") {
        return Ok(elem.clone());
    }
    if let Some(elem) = map.get("Numeric") {
        return Ok(elem.clone());
    }

    if let Some(elem) = map.get("Array") {
        return Ok(elem.clone());
    }

    Err(format!(
        "Builtin Typing bad argument type in question -- {:?} === {:?}",
        map,
        map.keys()
    ))
}

pub fn button<S: BuildHasher>(args: &HashMap<String, Literal, S>) -> Result<MessageType, String> {
    if args.len() == 1 {
        let elem = get_one_of(args)?;

        Ok(MessageType::Msg(
            Message {
                content_type: BUTTON.to_owned().to_lowercase(),
                content: parse_quickbutton(elem, Literal::StringLiteral("quick_buttons".to_owned())),
            }
        ))
    } else {
        Err("Builtin Button bad number of argument".to_owned())
    }
}

pub fn question<S: BuildHasher>(args: &HashMap<String, Literal, S>, name: String) -> Result<MessageType, String> {
    let mut question_value = HashMap::new();

    let expr_title = args.get("title").expect("error in question expr_title");
    let button_type = args.get("button_type").expect("error in question button_type");
    let expr_buttons = args.get("buttons").expect("error in question expr_buttons");

    let mut accepts: Vec<Literal> = vec![];
    let mut buttons: Vec<Literal> = vec![];
    if let Literal::ArrayLiteral(array) = expr_buttons {
        for button in array.iter() {
            match button {
                Literal::ObjectLiteral { name, value } if name == BUTTON => {
                    let elem = get_one_of(value)?;

                    accepts.push(elem.clone());
                    buttons.push(
                        parse_quickbutton(elem, button_type.clone())
                    )
                }
                err => return Err(format!("Builtin Typing bad argument type -> {:?}", err)),
            }
        }
        question_value.insert("title".to_owned(), expr_title.clone());
        question_value.insert("accepts".to_owned(), Literal::ArrayLiteral(accepts));
        question_value.insert("buttons".to_owned(), Literal::ArrayLiteral(buttons));
    };

    Ok(MessageType::Msg(Message {
        content_type: name.to_lowercase(),
        content: Literal::ObjectLiteral {
            name: "question".to_owned(),
            value: question_value,
        },
    }))
}
