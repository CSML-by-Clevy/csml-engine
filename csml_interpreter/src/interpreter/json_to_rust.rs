use crate::data::position::Position;
use crate::data::primitive::{
    PrimitiveArray, PrimitiveBoolean, PrimitiveClosure, PrimitiveFloat, PrimitiveInt,
    PrimitiveNull, PrimitiveObject, PrimitiveString,
};
use crate::data::{ast::Interval, Data, Literal, MessageData, MSG};
use crate::error_format::*;
use crate::parser::parse_string::interpolate_string;
use std::{collections::HashMap, sync::mpsc};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn interpolate(
    literal: &serde_json::Value,
    interval: Interval,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match literal {
        serde_json::Value::String(val) => interpolate_string(val, data, msg_data, sender),
        _ => json_to_literal(literal, interval),
    }
}

pub fn json_to_literal(
    literal: &serde_json::Value,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match literal {
        serde_json::Value::String(val) => Ok(PrimitiveString::get_literal(val, interval)),
        serde_json::Value::Bool(val) => Ok(PrimitiveBoolean::get_literal(*val, interval)),
        serde_json::Value::Null => Ok(PrimitiveNull::get_literal(interval)),
        serde_json::Value::Number(val) => {
            if let (true, Some(float)) = (val.is_f64(), val.as_f64()) {
                Ok(PrimitiveFloat::get_literal(float, interval))
            } else if let (true, Some(int)) = (val.is_i64(), val.as_i64()) {
                Ok(PrimitiveInt::get_literal(int, interval))
            } else {
                Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JSON_TO_LITERAL.to_owned(),
                ))
            }
        }
        serde_json::Value::Array(val) => {
            let mut vec = vec![];

            for elem in val {
                vec.push(json_to_literal(elem, interval)?);
            }

            Ok(PrimitiveArray::get_literal(&vec, interval))
        }
        serde_json::Value::Object(val) => {
            let mut map = HashMap::new();

            for (k, v) in val.iter() {
                map.insert(k.to_owned(), json_to_literal(v, interval)?);
            }

            Ok(PrimitiveObject::get_literal(&map, interval))
        }
    }
}

pub fn memory_to_literal(
    literal: &serde_json::Value,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match literal {
        serde_json::Value::String(val) => Ok(PrimitiveString::get_literal(val, interval)),
        serde_json::Value::Bool(val) => Ok(PrimitiveBoolean::get_literal(*val, interval)),
        serde_json::Value::Null => Ok(PrimitiveNull::get_literal(interval)),
        serde_json::Value::Number(val) => {
            if let (true, Some(float)) = (val.is_f64(), val.as_f64()) {
                Ok(PrimitiveFloat::get_literal(float, interval))
            } else if let (true, Some(int)) = (val.is_i64(), val.as_i64()) {
                Ok(PrimitiveInt::get_literal(int, interval))
            } else {
                Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JSON_TO_LITERAL.to_owned(),
                ))
            }
        }
        serde_json::Value::Array(val) => {
            let mut vec = vec![];

            for elem in val {
                vec.push(memory_to_literal(elem, interval)?);
            }

            Ok(PrimitiveArray::get_literal(&vec, interval))
        }
        serde_json::Value::Object(val) => {
            let mut map = HashMap::new();

            match (
                val.get("_content"),
                val.get("_content_type"),
                val.get("_closure"),
            ) {
                (Some(content), Some(serde_json::Value::String(conent_type)), _) => {
                    let mut literal = memory_to_literal(content, interval)?;
                    literal.set_content_type(&conent_type);
                    Ok(literal)
                }
                (_, _, Some(closure_json)) => {
                    let closure: PrimitiveClosure =
                        serde_json::from_value(closure_json.to_owned())?;

                    Ok(Literal {
                        content_type: "closure".to_owned(),
                        primitive: Box::new(closure),
                        interval,
                    })
                }
                (_, _, _) => {
                    for (k, v) in val.iter() {
                        map.insert(k.to_owned(), memory_to_literal(v, interval)?);
                    }
                    Ok(PrimitiveObject::get_literal(&map, interval))
                }
            }
        }
    }
}
