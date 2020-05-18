use crate::data::{ast::Interval, Literal};
use crate::error_format::*;
use std::collections::HashMap;

use crate::data::position::Position;
use crate::data::primitive::{
    array::PrimitiveArray, boolean::PrimitiveBoolean, float::PrimitiveFloat, int::PrimitiveInt,
    null::PrimitiveNull, object::PrimitiveObject, string::PrimitiveString,
};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

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
