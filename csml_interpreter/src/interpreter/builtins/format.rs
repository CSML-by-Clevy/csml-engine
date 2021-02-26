use crate::data::position::Position;
use crate::data::primitive::{object::PrimitiveObject, string::PrimitiveString, PrimitiveType};
use crate::data::{ast::Interval, ArgsType, Literal};
use crate::error_format::*;
use std::collections::HashMap;

pub fn debug(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
    Ok(args.args_to_debug(interval))
}

// TODO: old builtin need to be rm when no one use it
pub fn object(object: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut map = HashMap::new();

    object.populate(&mut map, &[], interval)?;

    Ok(PrimitiveObject::get_literal(&map, interval))
}

pub fn http(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut http: HashMap<String, Literal> = HashMap::new();
    let mut header = HashMap::new();

    match args.get("url", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            header.insert(
                "content-type".to_owned(),
                PrimitiveString::get_literal("application/json", interval),
            );
            header.insert(
                "accept".to_owned(),
                PrimitiveString::get_literal("application/json,text/*", interval),
            );
            header.insert(
                "User-Agent".to_owned(),
                PrimitiveString::get_literal("csml/v1", interval),
            );

            http.insert("url".to_owned(), literal.to_owned());
            http.insert(
                "method".to_owned(),
                PrimitiveString::get_literal("get", interval),
            );

            let lit_header = PrimitiveObject::get_literal(&header, interval);
            http.insert("header".to_owned(), lit_header);
            let lit_query = PrimitiveObject::get_literal(&HashMap::default(), interval);
            http.insert("query".to_owned(), lit_query);
            let lit_body = PrimitiveObject::get_literal(&HashMap::default(), interval);
            http.insert("body".to_owned(), lit_body);

            args.populate(&mut http, &["url", "header", "query", "body"], interval)?;

            let mut result = PrimitiveObject::get_literal(&http, interval);

            result.set_content_type("http");

            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval),
            ERROR_HTTP.to_owned(),
        )),
    }
}

pub fn base64(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("string", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            let mut object: HashMap<String, Literal> = HashMap::new();
            object.insert("string".to_owned(), literal.to_owned());

            let mut result = PrimitiveObject::get_literal(&object, interval);

            result.set_content_type("base64");

            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval),
            ERROR_HTTP.to_owned(),
        )),
    }
}

pub fn hex(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("string", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            let mut object: HashMap<String, Literal> = HashMap::new();
            object.insert("string".to_owned(), literal.to_owned());

            let mut result = PrimitiveObject::get_literal(&object, interval);

            result.set_content_type("hex");

            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval),
            ERROR_HTTP.to_owned(),
        )),
    }
}
