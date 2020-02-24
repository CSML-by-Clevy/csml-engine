use crate::data::primitive::array::PrimitiveArray;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::float::PrimitiveFloat;
use crate::data::primitive::int::PrimitiveInt;
use crate::data::primitive::null::PrimitiveNull;
use crate::data::primitive::object::PrimitiveObject;
use crate::data::primitive::tools::check_usage;
use crate::data::primitive::tools::*;
use crate::data::primitive::Right;
use crate::data::primitive::{Primitive, PrimitiveType};
use crate::data::{ast::Interval, memories::MemoryType, message::Message, Literal};
use crate::error_format::ErrorInfo;
use lazy_static::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        // type_of() -> Primitive<String>
        map.insert("type_of", (type_of as PrimitiveMethod, Right::Read));

        // to_string() -> Primitive<String>
        map.insert("to_string", (to_string as PrimitiveMethod, Right::Read));

        // append(Primitive<String>) -> Primitive<String>
        map.insert("append", (append as PrimitiveMethod, Right::Write));

        // match(Primitive<String>) -> Primitive<Array>
        map.insert("match", (do_match as PrimitiveMethod, Right::Read));

        // clear() -> Primitive<String>
        map.insert("clear", (clear as PrimitiveMethod, Right::Write));

        // length() -> Primitive<Int>
        map.insert("length", (length as PrimitiveMethod, Right::Read));

        // is_empty() -> Primitive<Boolean>
        map.insert("is_empty", (is_empty as PrimitiveMethod, Right::Read));

        // to_lower_case() -> Primitive<String>
        map.insert("to_lower_case", (to_lower_case as PrimitiveMethod, Right::Write));

        // to_upper_case() -> Primitive<String>
        map.insert("to_upper_case", (to_upper_case as PrimitiveMethod, Right::Write));

        // contains(Primitive<String>) -> Primitive<Boolean>
        map.insert("contains", (contains as PrimitiveMethod, Right::Read));

        // starts_with(Primitive<String>) -> Primitive<Boolean>
        map.insert("starts_with", (starts_with as PrimitiveMethod, Right::Read));

        // ends_with(Primitive<String>) -> Primitive<Boolean>
        map.insert("ends_with", (ends_with as PrimitiveMethod, Right::Read));

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveString {
    pub value: String,
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn type_of(
    _string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "type_of()", interval)?;

    Ok(PrimitiveString::get_literal("string", "string", interval))
}

fn to_string(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "to_string()", interval)?;

    Ok(PrimitiveString::get_literal(
        "string",
        &string.to_string(),
        interval,
    ))
}

fn append(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 1, "append(Primitive<String>)", interval)?;

    for literal in args.iter() {
        string.value.push_str(&literal.primitive.to_string());
    }

    Ok(PrimitiveNull::get_literal("null", interval))
}

fn do_match(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 1, "match(Primitive<String>)", interval)?;

    let mut s: &str = &string.value;
    let mut vector: Vec<Literal> = Vec::new();
    let args = match args.get(0) {
        Some(res) => res,
        None => {
            return Err(ErrorInfo {
                message: "usage: need to have one parameter".to_owned(),
                interval,
            });
        }
    };

    let pattern = match Literal::get_value::<String>(&args.primitive) {
        Ok(res) => res,
        Err(_) => {
            return Err(ErrorInfo {
                message: "usage: parameter must be of type string".to_owned(),
                interval,
            });
        }
    };

    let action = match Regex::new(pattern) {
        Ok(res) => res,
        Err(_) => {
            return Err(ErrorInfo {
                message: "usage: parameter must be a valid regex expression".to_owned(),
                interval,
            });
        }
    };

    while let Some(result) = action.find(&s) {
        vector.push(PrimitiveString::get_literal(
            "string",
            &s[result.start()..result.end()],
            interval,
        ));
        s = &s[result.end()..];
    }

    match vector.len() {
        0 => Ok(PrimitiveNull::get_literal("null", interval)),
        _ => Ok(PrimitiveArray::get_literal("array", &vector, interval)),
    }
}

fn clear(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "clear()", interval)?;

    string.value.clear();

    Ok(PrimitiveNull::get_literal("null", interval))
}

fn length(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "length()", interval)?;

    let result = string.value.len();

    Ok(PrimitiveInt::get_literal("int", result as i64, interval))
}

fn is_empty(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "is_empty()", interval)?;

    let result = string.value.is_empty();

    Ok(PrimitiveBoolean::get_literal("boolean", result, interval))
}

fn to_lower_case(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "to_lower_case()", interval)?;

    string.value.make_ascii_lowercase();

    Ok(PrimitiveNull::get_literal("null", interval))
}

fn to_upper_case(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "to_upper_case()", interval)?;

    string.value.make_ascii_uppercase();

    Ok(PrimitiveNull::get_literal("null", interval))
}

fn contains(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 1, "contains(Primitive<String>)", interval)?;

    let args = match args.get(0) {
        Some(res) => res,
        None => {
            return Err(ErrorInfo {
                message: "usage: need to have one parameter".to_owned(),
                interval,
            });
        }
    };

    let pattern = match Literal::get_value::<String>(&args.primitive) {
        Ok(res) => res,
        Err(_) => {
            return Err(ErrorInfo {
                message: "usage: parameter must be of type string".to_owned(),
                interval,
            });
        }
    };

    let action = match Regex::new(pattern) {
        Ok(res) => res,
        Err(_) => {
            return Err(ErrorInfo {
                message: "usage: parameter must be a valid regex expression".to_owned(),
                interval,
            });
        }
    };

    let result = action.is_match(&string.value);

    Ok(PrimitiveBoolean::get_literal("boolean", result, interval))
}

fn starts_with(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 1, "starts_with(Primitive<String>)", interval)?;

    let args = match args.get(0) {
        Some(res) => res,
        None => {
            return Err(ErrorInfo {
                message: "usage: need to have one parameter".to_owned(),
                interval,
            });
        }
    };

    let pattern = match Literal::get_value::<String>(&args.primitive) {
        Ok(res) => res,
        Err(_) => {
            return Err(ErrorInfo {
                message: "usage: parameter must be of type string".to_owned(),
                interval,
            });
        }
    };

    let action = match Regex::new(pattern) {
        Ok(res) => res,
        Err(_) => {
            return Err(ErrorInfo {
                message: "usage: parameter must be a valid regex expression".to_owned(),
                interval,
            });
        }
    };

    if let Some(res) = action.find(&string.value) {
        if res.start() == 0 {
            return Ok(PrimitiveBoolean::get_literal("boolean", true, interval));
        }
    }

    Ok(PrimitiveBoolean::get_literal("boolean", false, interval))
}

fn ends_with(
    string: &mut PrimitiveString,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 1, "starts_with(Primitive<String>)", interval)?;

    let args = match args.get(0) {
        Some(res) => res,
        None => {
            return Err(ErrorInfo {
                message: "usage: need to have one parameter".to_owned(),
                interval,
            });
        }
    };

    let pattern = match Literal::get_value::<String>(&args.primitive) {
        Ok(res) => res,
        Err(_) => {
            return Err(ErrorInfo {
                message: "usage: parameter must be of type string".to_owned(),
                interval,
            });
        }
    };

    let action = match Regex::new(pattern) {
        Ok(res) => res,
        Err(_) => {
            return Err(ErrorInfo {
                message: "usage: parameter must be a valid regex expression".to_owned(),
                interval,
            });
        }
    };

    if let Some(res) = action.find(&string.value) {
        if res.end() == string.value.len() {
            return Ok(PrimitiveBoolean::get_literal("boolean", true, interval));
        }
    }

    Ok(PrimitiveBoolean::get_literal("boolean", false, interval))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveString {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    pub fn get_literal(content_type: &str, string: &str, interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveString::new(string));

        Literal {
            content_type: content_type.to_owned(),
            primitive,
            interval,
        }
    }
}

impl Primitive for PrimitiveString {
    fn do_exec(
        &mut self,
        name: &str,
        args: &[Literal],
        interval: Interval,
        _mem_type: &MemoryType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        if let Some((f, right)) = FUNCTIONS.get(name) {
            let res = f(self, args, interval)?;

            return Ok((res, *right));
        }

        Err(ErrorInfo {
            message: format!("unknown method '{}' for type String", name),
            interval,
        })
    }

    fn is_eq(&self, other: &dyn Primitive) -> bool {
        let rhs = if let Some(rhs) = other.as_any().downcast_ref::<PrimitiveString>() {
            rhs
        } else {
            return false;
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => lhs == rhs,
            (Ok(Integer::Float(lhs)), Ok(Integer::Float(rhs))) => lhs == rhs,
            (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => (lhs as f64) == rhs,
            (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => lhs == (rhs as f64),
            _ => self.value == rhs.value,
        }
    }

    fn is_cmp(&self, other: &dyn Primitive) -> Option<Ordering> {
        let rhs = if let Some(rhs) = other.as_any().downcast_ref::<PrimitiveString>() {
            rhs
        } else {
            return None;
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => lhs.partial_cmp(&rhs),
            (Ok(Integer::Float(lhs)), Ok(Integer::Float(rhs))) => lhs.partial_cmp(&rhs),
            (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => (lhs as f64).partial_cmp(&rhs),
            (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => lhs.partial_cmp(&(rhs as f64)),
            _ => self.value.partial_cmp(&rhs.value),
        }
    }

    fn do_add(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "rhs need to be of type string".to_owned(),
                    interval: Interval { column: 0, line: 0 },
                });
            }
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveInt::new(lhs + rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs + rhs)))
            }
            (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs as f64 + rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs + rhs as f64)))
            }
            _ => Err(ErrorInfo {
                message: "[!] Add: Illegal operation".to_owned(),
                interval: Interval { column: 0, line: 0 },
            }),
        }
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "rhs need to be of type string".to_owned(),
                    interval: Interval { column: 0, line: 0 },
                });
            }
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveInt::new(lhs - rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs - rhs)))
            }
            (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs as f64 - rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs - rhs as f64)))
            }
            _ => Err(ErrorInfo {
                message: "[!] Sub: Illegal operation".to_owned(),
                interval: Interval { column: 0, line: 0 },
            }),
        }
    }

    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "rhs need to be of type string".to_owned(),
                    interval: Interval { column: 0, line: 0 },
                });
            }
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => {
                check_division_by_zero_i64(lhs, rhs)?;

                Ok(Box::new(PrimitiveInt::new(lhs / rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Float(rhs))) => {
                check_division_by_zero_i64(lhs as i64, rhs as i64)?;

                Ok(Box::new(PrimitiveFloat::new(lhs / rhs)))
            }
            (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => {
                check_division_by_zero_i64(lhs, rhs as i64)?;

                Ok(Box::new(PrimitiveFloat::new(lhs as f64 / rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => {
                check_division_by_zero_i64(lhs as i64, rhs)?;

                Ok(Box::new(PrimitiveFloat::new(lhs / rhs as f64)))
            }
            _ => Err(ErrorInfo {
                message: "[!] Div: Illegal operation".to_owned(),
                interval: Interval { column: 0, line: 0 },
            }),
        }
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "rhs need to be of type string".to_owned(),
                    interval: Interval { column: 0, line: 0 },
                });
            }
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveInt::new(lhs * rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs * rhs)))
            }
            (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs as f64 * rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs * rhs as f64)))
            }
            _ => Err(ErrorInfo {
                message: "[!] Mul: Illegal operation".to_owned(),
                interval: Interval { column: 0, line: 0 },
            }),
        }
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "rhs need to be of type string".to_owned(),
                    interval: Interval { column: 0, line: 0 },
                });
            }
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveInt::new(lhs * rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs * rhs)))
            }
            (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs as f64 * rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs * rhs as f64)))
            }
            _ => Err(ErrorInfo {
                message: "[!] Rem: Illegal operation".to_owned(),
                interval: Interval { column: 0, line: 0 },
            }),
        }
    }

    fn do_bitand(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "rhs need to be of type string".to_owned(),
                    interval: Interval { column: 0, line: 0 },
                });
            }
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveInt::new(lhs & rhs)))
            }
            _ => Err(ErrorInfo {
                message: "[!] BitAnd: Illegal operation".to_owned(),
                interval: Interval { column: 0, line: 0 },
            }),
        }
    }

    fn do_bitor(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        let lhs = match self.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "rhs need to be of type string".to_owned(),
                    interval: Interval { column: 0, line: 0 },
                });
            }
        };
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "rhs need to be of type string".to_owned(),
                    interval: Interval { column: 0, line: 0 },
                });
            }
        };

        match (get_integer(&lhs.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveInt::new(lhs | rhs)))
            }
            _ => Err(ErrorInfo {
                message: "[!] BitOr: Illegal operation".to_owned(),
                interval: Interval { column: 0, line: 0 },
            }),
        }
    }

    fn as_debug(&self) -> &dyn std::fmt::Debug {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::PrimitiveString
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self.value)
    }

    fn to_string(&self) -> String {
        self.value.to_owned()
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

    fn to_msg(&self, _content_type: String) -> Message {
        let mut hashmap: HashMap<String, Literal> = HashMap::new();

        hashmap.insert(
            "text".to_owned(),
            Literal {
                content_type: "string".to_owned(),
                primitive: Box::new(PrimitiveString::new(&self.value)),
                interval: Interval { column: 0, line: 0 },
            },
        );

        let result =
            PrimitiveObject::get_literal("text", &hashmap, Interval { column: 0, line: 0 });

        Message {
            content_type: result.content_type,
            content: result.primitive.to_json(),
        }
    }
}
