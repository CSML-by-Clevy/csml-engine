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
        _ => json_to_literal(literal, interval, &data.context.flow),
    }
}

pub fn json_to_literal(
    literal: &serde_json::Value,
    interval: Interval,
    flow_name: &str,
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
                    Position::new(interval, flow_name),
                    ERROR_JSON_TO_LITERAL.to_owned(),
                ))
            }
        }
        serde_json::Value::Array(val) => {
            let mut vec = vec![];

            for elem in val {
                vec.push(json_to_literal(elem, interval, flow_name)?);
            }

            Ok(PrimitiveArray::get_literal(&vec, interval))
        }
        serde_json::Value::Object(val) => {
            let mut map = HashMap::new();

            for (k, v) in val.iter() {
                map.insert(k.to_owned(), json_to_literal(v, interval, flow_name)?);
            }

            Ok(PrimitiveObject::get_literal(&map, interval))
        }
    }
}

pub fn memory_to_literal(
    literal: &serde_json::Value,
    interval: Interval,
    flow_name: &str,
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
                    Position::new(interval, flow_name),
                    ERROR_JSON_TO_LITERAL.to_owned(),
                ))
            }
        }
        serde_json::Value::Array(val) => {
            let mut vec = vec![];

            for elem in val {
                vec.push(memory_to_literal(elem, interval, flow_name)?);
            }

            Ok(PrimitiveArray::get_literal(&vec, interval))
        }

        serde_json::Value::Object(map) if map.contains_key("_additional_info") => {
            if let (Some(value), Some(serde_json::Value::Object(additional_info))) =
                (map.get("value"), map.get("_additional_info"))
            {
                let mut literal = memory_to_literal(value, interval, flow_name)?;

                for (k, v) in additional_info.iter() {
                    literal.add_info(k, memory_to_literal(v, interval, flow_name)?);
                }

                Ok(literal)
            } else {
                Ok(PrimitiveNull::get_literal(interval))
            }
        }

        serde_json::Value::Object(map)
            if map.contains_key("_content") && map.contains_key("_content_type") =>
        {
            if let (Some(content), Some(serde_json::Value::String(conent_type))) =
                (map.get("_content"), map.get("_content_type"))
            {
                let mut literal = memory_to_literal(content, interval, flow_name)?;
                literal.set_content_type(&conent_type);
                Ok(literal)
            } else {
                Ok(PrimitiveNull::get_literal(interval))
            }
        }

        serde_json::Value::Object(map) if map.contains_key("_closure") => {
            if let Some(closure_json) = map.get("_closure") {
                let closure: PrimitiveClosure = serde_json::from_value(closure_json.to_owned())?;

                Ok(Literal {
                    content_type: "closure".to_owned(),
                    primitive: Box::new(closure),
                    additional_info: None,
                    interval,
                })
            } else {
                Ok(PrimitiveNull::get_literal(interval))
            }
        }

        serde_json::Value::Object(map) => {
            let mut obj = HashMap::new();

            for (k, v) in map.iter() {
                obj.insert(k.to_owned(), memory_to_literal(v, interval, flow_name)?);
            }
            Ok(PrimitiveObject::get_literal(&obj, interval))
        }
    }
}
