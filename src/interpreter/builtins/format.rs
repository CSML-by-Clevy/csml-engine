use crate::data::{ast::Interval, tokens::*, Literal};
use crate::error_format::ErrorInfo;
use crate::interpreter::builtins::tools::*;
use crate::data::primitive::{object::PrimitiveObject, string::PrimitiveString, PrimitiveType};
use std::collections::HashMap;

pub fn text(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => Ok(PrimitiveString::get_literal(
            "string",
            &literal.primitive.to_string(),
            literal.interval,
        )),
        _ => Err(ErrorInfo {
            message: "Builtin Text expect one argument of type string | example: Text(\"hola\")"
                .to_owned(),
            interval,
        }),
    }
}

pub fn object(object: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    Ok(PrimitiveObject::get_literal("object", &object, interval))
}

pub fn question(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut question: HashMap<String, Literal> = HashMap::new();

    let buttons = match args.get("buttons") {
        Some(literal) => literal.to_owned(),
        _ => {
            return Err(ErrorInfo {
            message: "argument buttons in Builtin Question need to be of type Array of Button Component example: [ Button(\"b1\"), Button(\"b2\") ]".to_owned(),
            interval,
            })
        }
    };

    let accepts = accepts_from_buttons(&buttons);

    if let Ok(title) = search_or_default(&args, "title", interval, None) {
        question.insert("title".to_owned(), title);
    }

    question.insert("accepts".to_owned(), accepts);
    question.insert("buttons".to_owned(), buttons);

    Ok(PrimitiveObject::get_literal(
        "question", &question, interval,
    ))
}

// TODO: check nbr elemts in built-ins
pub fn typing(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut typing: HashMap<String, Literal> = HashMap::new();

    match args.get(DEFAULT) {
        Some(literal)
            if literal.primitive.get_type() == PrimitiveType::PrimitiveInt
                || literal.primitive.get_type() == PrimitiveType::PrimitiveFloat =>
        {
            typing.insert("duration".to_owned(), literal.to_owned());
            Ok(PrimitiveObject::get_literal("typing", &typing, interval))
        }
        _ => Err(ErrorInfo {
            message:
                "Builtin Typing expect one argument of type int or float | example: Typing(3, ..)"
                    .to_owned(),
            interval,
        }),
    }
}

pub fn wait(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut wait: HashMap<String, Literal> = HashMap::new();

    match args.get(DEFAULT) {
        Some(literal)
            if literal.primitive.get_type() == PrimitiveType::PrimitiveInt
                || literal.primitive.get_type() == PrimitiveType::PrimitiveFloat =>
        {
            wait.insert("duration".to_owned(), literal.to_owned());

            Ok(PrimitiveObject::get_literal("wait", &wait, interval))
        }
        _ => Err(ErrorInfo {
            message: "Builtin Wait expect one argument of type int or float | example: Wait(3)"
                .to_owned(),
            interval,
        }),
    }
}
