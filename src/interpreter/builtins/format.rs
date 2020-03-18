use crate::data::primitive::{object::PrimitiveObject, string::PrimitiveString, PrimitiveType};
use crate::data::{ast::Interval, tokens::*, Literal};
use crate::error_format::ErrorInfo;
use crate::interpreter::builtins::tools::*;
use std::collections::HashMap;

pub fn text(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => Ok(PrimitiveString::get_literal(
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
    Ok(PrimitiveObject::get_literal(&object, interval))
}

pub fn question(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
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

    let mut result = PrimitiveObject::get_literal(&question, interval);
    result.set_content_type("question");

    Ok(result)
}

// TODO: check nbr elemts in built-ins
pub fn typing(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut typing: HashMap<String, Literal> = HashMap::new();

    match args.get(DEFAULT) {
        Some(literal)
            if literal.primitive.get_type() == PrimitiveType::PrimitiveInt
                || literal.primitive.get_type() == PrimitiveType::PrimitiveFloat =>
        {
            typing.insert("duration".to_owned(), literal.to_owned());

            let mut result = PrimitiveObject::get_literal(&typing, interval);
            result.set_content_type("typing");

            Ok(result)
        }
        _ => Err(ErrorInfo {
            message:
                "Builtin Typing expect one argument of type int or float | example: Typing(3, ..)"
                    .to_owned(),
            interval,
        }),
    }
}

pub fn wait(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut wait: HashMap<String, Literal> = HashMap::new();

    match args.get(DEFAULT) {
        Some(literal)
            if literal.primitive.get_type() == PrimitiveType::PrimitiveInt
                || literal.primitive.get_type() == PrimitiveType::PrimitiveFloat =>
        {
            wait.insert("duration".to_owned(), literal.to_owned());

            let mut result = PrimitiveObject::get_literal(&wait, interval);
            result.set_content_type("wait");

            Ok(result)
        }
        _ => Err(ErrorInfo {
            message: "Builtin Wait expect one argument of type int or float | example: Wait(3)"
                .to_owned(),
            interval,
        }),
    }
}

pub fn http(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut http: HashMap<String, Literal> = HashMap::new();
    let mut header = HashMap::new();

    match args.get(DEFAULT) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            header.insert(
                "content-type".to_owned(),
                PrimitiveString::get_literal("application/json", interval),
            );
            header.insert(
                "accept".to_owned(),
                PrimitiveString::get_literal("application/json,text/*", interval),
            );

            http.insert("url".to_owned(), literal.to_owned());
            http.insert(
                "method".to_owned(),
                PrimitiveString::get_literal("get", interval),
            );

            http.insert(
                "header".to_owned(),
                PrimitiveObject::get_literal(&header, interval),
            );
            http.insert(
                "query".to_owned(),
                PrimitiveObject::get_literal(&HashMap::default(), interval),
            );
            http.insert(
                "body".to_owned(),
                PrimitiveObject::get_literal(&HashMap::default(), interval),
            );

            let mut result = PrimitiveObject::get_literal(&http, interval);

            result.set_content_type("http");

            Ok(result)
        }
        _ => Err(ErrorInfo {
            message:
                "Builtin HTTP expect one url of type string | example: HTTP(\"https://clevy.io\")"
                    .to_owned(),
            interval,
        }),
    }
}
