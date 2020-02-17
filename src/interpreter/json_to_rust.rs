use crate::data::{ast::Interval, Literal};
use crate::error_format::ErrorInfo;
use std::collections::HashMap;

use crate::data::primitive::{
    array::PrimitiveArray, boolean::PrimitiveBoolean, float::PrimitiveFloat, int::PrimitiveInt,
    null::PrimitiveNull, object::PrimitiveObject, string::PrimitiveString,
};

pub fn json_to_literal(
    literal: &serde_json::Value,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match literal {
        serde_json::Value::String(val) => Ok(PrimitiveString::get_literal("string", val, interval)),
        serde_json::Value::Bool(val) => {
            Ok(PrimitiveBoolean::get_literal("boolean", *val, interval))
        }
        serde_json::Value::Null => Ok(PrimitiveNull::get_literal("null", interval)),
        serde_json::Value::Number(val) => {
            if let (true, Some(float)) = (val.is_f64(), val.as_f64()) {
                Ok(PrimitiveFloat::get_literal("float", float, interval))
            } else if let (true, Some(int)) = (val.is_i64(), val.as_i64()) {
                Ok(PrimitiveInt::get_literal("int", int, interval))
            } else {
                Err(ErrorInfo {
                    message: format!("Number of type {} bad format", val),
                    interval,
                })
            }
        }
        serde_json::Value::Array(val) => {
            let mut vec = vec![];

            for elem in val {
                vec.push(json_to_literal(elem, interval)?);
            }

            Ok(PrimitiveArray::get_literal("array", &vec, interval))
        }
        serde_json::Value::Object(val) => {
            let mut map = HashMap::new();

            for (k, v) in val.iter() {
                map.insert(k.to_owned(), json_to_literal(v, interval)?);
            }

            Ok(PrimitiveObject::get_literal("object", &map, interval))
        }
    }
}
