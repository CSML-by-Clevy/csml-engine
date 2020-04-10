use crate::data::primitive::{object::PrimitiveObject, string::PrimitiveString, PrimitiveType};
use crate::data::{ast::Interval, tokens::DEFAULT, Literal};
use crate::error_format::*;
use crate::interpreter::builtins::tools::*;
use std::collections::HashMap;

pub fn text(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => Ok(PrimitiveString::get_literal(
            &literal.primitive.to_string(),
            literal.interval,
        )),
        _ => Err(gen_error_info(interval, ERROR_TEXT.to_owned())),
    }
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
        _ => Err(gen_error_info(interval, ERROR_TYPING.to_owned())),
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
        _ => Err(gen_error_info(interval, ERROR_WAIT.to_owned())),
    }
}

// TODO: old builtin need to rm this whene no one use itold built in need to rm this when no one use it
pub fn object(object: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    Ok(PrimitiveObject::get_literal(&object, interval))
}

pub fn question(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut question: HashMap<String, Literal> = HashMap::new();

    let buttons = match args.get("buttons") {
        Some(literal) => literal.to_owned(),
        _ => return Err(gen_error_info(interval, ERROR_QUESTION.to_owned())),
    };

    let accepts = accepts_from_buttons(&buttons);

    match (args.get("title"), args.get(DEFAULT)) {
        (Some(title), ..) | (.., Some(title)) => {
            question.insert("title".to_owned(), title.to_owned());
        }
        _ => {}
    }

    question.insert("accepts".to_owned(), accepts);
    question.insert("buttons".to_owned(), buttons);

    let mut result = PrimitiveObject::get_literal(&question, interval);
    result.set_content_type("question");

    Ok(result)
}

pub fn carousel(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut carousel: HashMap<String, Literal> = HashMap::new();

    let cards = match args.get("cards") {
        Some(literal) => literal.to_owned(),
        _ => return Err(gen_error_info(interval, ERROR_CAROUSEL.to_owned())),
    };

    if let Some(literal) = args.get("metadata") {
        carousel.insert("metadata".to_owned(), literal.to_owned());
    }

    carousel.insert("cards".to_owned(), cards);

    let mut result = PrimitiveObject::get_literal(&carousel, interval);
    result.set_content_type("carousel");

    Ok(result)
}

pub fn http(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut http: HashMap<String, Literal> = HashMap::new();
    let mut header = HashMap::new();

    match args.get(DEFAULT) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            header.insert(
                "content_type".to_owned(),
                PrimitiveString::get_literal("header", interval),
            );
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

            let mut lit_header = PrimitiveObject::get_literal(&header, interval);
            lit_header.set_content_type("header");
            http.insert(
                "header".to_owned(),
                lit_header,
            );
            let mut lit_query = PrimitiveObject::get_literal(&HashMap::default(), interval);
            lit_query.set_content_type("query");
            http.insert(
                "query".to_owned(),
                lit_query,
            );
            let mut lit_body = PrimitiveObject::get_literal(&HashMap::default(), interval);
            lit_body.set_content_type("body");
            http.insert(
                "body".to_owned(),
                lit_body,
            );

            let mut result = PrimitiveObject::get_literal(&http, interval);

            result.set_content_type("http");

            Ok(result)
        }
        _ => Err(gen_error_info(interval, ERROR_HTTP.to_owned())),
    }
}
