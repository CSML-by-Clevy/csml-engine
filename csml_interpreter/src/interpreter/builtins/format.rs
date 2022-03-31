use crate::data::position::Position;
use crate::data::primitive::{object::PrimitiveObject, PrimitiveType};
use crate::data::{ast::Interval, ArgsType, Literal};
use crate::error_format::*;
use std::collections::HashMap;

pub fn debug(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
    Ok(args.args_to_debug(interval))
}

// TODO: old builtin need to be rm when no one use it
pub fn object(object: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut map = HashMap::new();

    object.populate(&mut map, &[], flow_name, interval)?;

    Ok(PrimitiveObject::get_literal(&map, interval))
}

pub fn base64(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("string", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            let mut object: HashMap<String, Literal> = HashMap::new();
            object.insert("string".to_owned(), literal.to_owned());

            let mut result = PrimitiveObject::get_literal(&object, interval);

            result.set_content_type("base64");

            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_HTTP.to_owned(),
        )),
    }
}

pub fn hex(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("string", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            let mut object: HashMap<String, Literal> = HashMap::new();
            object.insert("string".to_owned(), literal.to_owned());

            let mut result = PrimitiveObject::get_literal(&object, interval);

            result.set_content_type("hex");

            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_HTTP.to_owned(),
        )),
    }
}
