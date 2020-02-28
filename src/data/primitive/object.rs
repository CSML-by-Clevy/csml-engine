use crate::data::primitive::array::PrimitiveArray;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::int::PrimitiveInt;
use crate::data::primitive::null::PrimitiveNull;
use crate::data::primitive::string::PrimitiveString;
use crate::data::primitive::tools::check_usage;
use crate::data::primitive::Right;
use crate::data::primitive::{Primitive, PrimitiveType};
use crate::data::{ast::Interval, memories::MemoryType, message::Message, Literal};
use crate::error_format::ErrorInfo;
use lazy_static::*;
use std::cmp::Ordering;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    object: &mut PrimitiveObject,
    args: &[Literal],
    interval: Interval,
    content_type: &str,
) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS_EVENT: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "get_type",
            (PrimitiveObject::get_type as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "get_metadata",
            (
                PrimitiveObject::get_metadata as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "type_of",
            (PrimitiveObject::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveObject::to_string as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "contains",
            (PrimitiveObject::contains as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_empty",
            (PrimitiveObject::is_empty as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "length",
            (PrimitiveObject::length as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "keys",
            (PrimitiveObject::keys as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "values",
            (PrimitiveObject::values as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_number",
            (
                PrimitiveObject::is_number_event as PrimitiveMethod,
                Right::Read,
            ),
        );

        map
    };
}

lazy_static! {
    static ref FUNCTIONS_LIB: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "type_of",
            (PrimitiveObject::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveObject::to_string as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "clear",
            (PrimitiveObject::clear as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "clear_values",
            (
                PrimitiveObject::clear_values as PrimitiveMethod,
                Right::Write,
            ),
        );
        map.insert(
            "contains",
            (PrimitiveObject::contains as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "insert",
            (PrimitiveObject::insert as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "remove",
            (PrimitiveObject::remove as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "is_empty",
            (PrimitiveObject::is_empty as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "length",
            (PrimitiveObject::length as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "keys",
            (PrimitiveObject::keys as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "values",
            (PrimitiveObject::values as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_number",
            (
                PrimitiveObject::is_number_object as PrimitiveMethod,
                Right::Read,
            ),
        );

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveObject {
    pub value: HashMap<String, Literal>,
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn get_key(literal: &Literal, interval: Interval) -> Result<String, ErrorInfo> {
    match literal.primitive.get_type() {
        PrimitiveType::PrimitiveString => Ok(literal.primitive.to_string()),
        _ => Err(ErrorInfo {
            message: "usage: key must be of type string".to_owned(),
            interval,
        }),
    }
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveObject {
    fn type_of(
        _object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "type_of()", interval)?;

        Ok(PrimitiveString::get_literal("object", interval))
    }

    fn to_string(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_string()", interval)?;

        Ok(PrimitiveString::get_literal(&object.to_string(), interval))
    }

    fn clear(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "clear()", interval)?;

        object.value.clear();

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn clear_values(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "clear_values()", interval)?;

        let mut vector: Vec<String> = Vec::new();

        for key in object.value.keys() {
            vector.push(key.to_owned());
        }

        for key in vector.iter() {
            object
                .value
                .insert(key.to_owned(), PrimitiveNull::get_literal(interval));
        }

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn contains(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "contains(Primitive<String>)", interval)?;

        let literal = match args.get(0) {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "usage: need to have one parameter".to_owned(),
                    interval,
                });
            }
        };

        let key = get_key(literal, interval)?;
        let result = object.value.contains_key(&key);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn insert(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 2, "insert(Primitive<String>, Primitive<T>)", interval)?;

        let (literal, value) = match (args.get(0), args.get(1)) {
            (Some(lhs), Some(rhs)) => (lhs, rhs),
            _ => {
                return Err(ErrorInfo {
                    message: "usage: need to have two parameters".to_owned(),
                    interval,
                });
            }
        };

        let key = get_key(literal, interval)?;

        match object.value.insert(key, value.to_owned()) {
            _ => Ok(PrimitiveNull::get_literal(interval)),
        }
    }

    fn remove(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "remove(Primitive<String>)", interval)?;

        let literal = match args.get(0) {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "usage: need to have one parameter".to_owned(),
                    interval,
                });
            }
        };

        let key = get_key(literal, interval)?;

        match object.value.remove(&key) {
            Some(value) => Ok(value),
            None => Ok(PrimitiveNull::get_literal(interval)),
        }
    }

    fn is_empty(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "is_empty()", interval)?;

        let result = object.value.is_empty();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn length(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "length()", interval)?;

        let result = object.value.len();

        Ok(PrimitiveInt::get_literal(result as i64, interval))
    }

    fn keys(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "keys()", interval)?;

        let mut result = Vec::new();

        for key in object.value.keys() {
            result.push(PrimitiveString::get_literal(key, interval));
        }

        Ok(PrimitiveArray::get_literal(&result, interval))
    }

    fn values(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "values()", interval)?;

        let mut result = Vec::new();

        for value in object.value.values() {
            result.push(value.to_owned());
        }

        Ok(PrimitiveArray::get_literal(&result, interval))
    }

    fn get_type(
        _object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "get_type()", interval)?;

        Ok(PrimitiveString::get_literal(content_type, interval))
    }

    fn get_metadata(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "get_metadata()", interval)?;

        Ok(Literal {
            content_type: content_type.to_owned(),
            primitive: Box::new(object.clone()),
            interval,
        })
    }

    fn is_number_event(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "is_number()", interval)?;

        if let Some(res) = object.value.get("text") {
            let result = res.primitive.to_string();
            let result = result.parse::<f64>().is_ok();

            return Ok(PrimitiveBoolean::get_literal(result, interval));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn is_number_object(
        _object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "is_number()", interval)?;

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveObject {
    pub fn new(value: &HashMap<String, Literal>) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    pub fn get_literal(object: &HashMap<String, Literal>, interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveObject::new(object));

        Literal {
            content_type: "object".to_owned(),
            primitive,
            interval,
        }
    }
}

impl Primitive for PrimitiveObject {
    fn do_exec(
        &mut self,
        name: &str,
        args: &[Literal],
        interval: Interval,
        mem_type: &MemoryType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        match mem_type {
            MemoryType::Event(content_type) => {
                if let Some((f, right)) = FUNCTIONS_EVENT.get(name) {
                    let res = f(self, args, interval, content_type)?;

                    return Ok((res, *right));
                }
            }
            _ => {
                if let Some((f, right)) = FUNCTIONS_LIB.get(name) {
                    let res = f(self, args, interval, "")?;

                    return Ok((res, *right));
                }
            }
        }

        Err(ErrorInfo {
            message: format!("unknown method '{}' for type Object", name),
            interval,
        })
    }

    fn is_eq(&self, other: &dyn Primitive) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.value == other.value
        } else {
            false
        }
    }

    fn is_cmp(&self, _other: &dyn Primitive) -> Option<Ordering> {
        None
    }

    fn do_add(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] Add: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_sub(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] Sub: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_div(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] Div: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_mul(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] Mul: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_rem(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] Rem: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_bitand(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] BitAnd: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_bitor(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] BitOr: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn as_debug(&self) -> &dyn std::fmt::Debug {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::PrimitiveObject
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> serde_json::Value {
        let mut object: serde_json::map::Map<String, serde_json::Value> =
            serde_json::map::Map::new();

        for (key, literal) in self.value.iter() {
            object.insert(key.to_owned(), literal.primitive.to_json());
        }

        serde_json::Value::Object(object)
    }

    fn to_string(&self) -> String {
        self.to_json().to_string()
    }

    fn as_bool(&self) -> bool {
        true
    }

    fn get_value(&self) -> &dyn std::any::Any {
        &self.value
    }

    fn get_mut_value(&mut self) -> &mut dyn std::any::Any {
        &mut self.value
    }

    fn to_msg(&self, content_type: String) -> Message {
        Message {
            content_type,
            content: self.to_json(),
        }
    }
}
