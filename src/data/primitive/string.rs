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

        map.insert(
            "type_of",
            (PrimitiveString::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveString::to_string as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "append",
            (PrimitiveString::append as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "match",
            (PrimitiveString::do_match as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "clear",
            (PrimitiveString::clear as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "length",
            (PrimitiveString::length as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_empty",
            (PrimitiveString::is_empty as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_lowercase",
            (
                PrimitiveString::to_lowercase as PrimitiveMethod,
                Right::Write,
            ),
        );
        map.insert(
            "to_uppercase",
            (
                PrimitiveString::to_uppercase as PrimitiveMethod,
                Right::Write,
            ),
        );
        map.insert(
            "contains",
            (PrimitiveString::contains as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "starts_with",
            (PrimitiveString::starts_with as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "ends_with",
            (PrimitiveString::ends_with as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "contains_regex",
            (
                PrimitiveString::contains_regex as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "starts_with_regex",
            (
                PrimitiveString::starts_with_regex as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "ends_with_regex",
            (
                PrimitiveString::ends_with_regex as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "match_regex",
            (
                PrimitiveString::do_match_regex as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "is_number",
            (PrimitiveString::is_number as PrimitiveMethod, Right::Read),
        );

        map.insert(
            "abs",
            (PrimitiveString::abs as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "cos",
            (PrimitiveString::cos as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "pow",
            (PrimitiveString::pow as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "floor",
            (PrimitiveString::floor as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "ceil",
            (PrimitiveString::ceil as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "round",
            (PrimitiveString::round as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "sin",
            (PrimitiveString::sin as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "sqrt",
            (PrimitiveString::sqrt as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "tan",
            (PrimitiveString::tan as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_number",
            (PrimitiveString::is_number as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_int",
            (PrimitiveString::to_int as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_float",
            (PrimitiveString::to_float as PrimitiveMethod, Right::Read),
        );

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveString {
    pub value: String,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveString {
    fn type_of(
        _string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "type_of()", interval)?;

        Ok(PrimitiveString::get_literal("string", interval))
    }

    fn to_string(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_string()", interval)?;

        Ok(PrimitiveString::get_literal(&string.to_string(), interval))
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

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn do_match(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "match(Primitive<String>)", interval)?;

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

        for result in string.value.matches(pattern) {
            vector.push(PrimitiveString::get_literal(result, interval));
        }

        if vector.is_empty() {
            return Ok(PrimitiveNull::get_literal(interval));
        }

        Ok(PrimitiveArray::get_literal(&vector, interval))
    }

    fn clear(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "clear()", interval)?;

        string.value.clear();

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn length(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "length()", interval)?;

        let result = string.value.len();

        Ok(PrimitiveInt::get_literal(result as i64, interval))
    }

    fn is_empty(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "is_empty()", interval)?;

        let result = string.value.is_empty();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn to_lowercase(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_lowercase()", interval)?;

        string.value.make_ascii_lowercase();

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn to_uppercase(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_uppercase()", interval)?;

        string.value.make_ascii_uppercase();

        Ok(PrimitiveNull::get_literal(interval))
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

        let result = string.value.contains(pattern);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn contains_regex(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "contains_regex(Primitive<String>)", interval)?;

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

        Ok(PrimitiveBoolean::get_literal(result, interval))
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

        let result = string.value.starts_with(pattern);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn starts_with_regex(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "starts_with_regex(Primitive<String>)", interval)?;

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
                return Ok(PrimitiveBoolean::get_literal(true, interval));
            }
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn ends_with(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "ends_with(Primitive<String>)", interval)?;

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

        let result = string.value.ends_with(pattern);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn ends_with_regex(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "ends_with_regex(Primitive<String>)", interval)?;

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
                return Ok(PrimitiveBoolean::get_literal(true, interval));
            }
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn do_match_regex(
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
                &s[result.start()..result.end()],
                interval,
            ));
            s = &s[result.end()..];
        }

        if vector.is_empty() {
            return Ok(PrimitiveNull::get_literal(interval));
        }

        Ok(PrimitiveArray::get_literal(&vector, interval))
    }

    fn is_number(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "is_number()", interval)?;

        let result = string.value.parse::<f64>().is_ok();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn abs(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "abs()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => {
                let result = res.abs();

                Ok(PrimitiveString::get_literal(&result.to_string(), interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn pow(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "pow(Primitive<Int || Float>)", interval)?;

        let literal = match args.get(0) {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "usage: need to have one parameter".to_owned(),
                    interval,
                });
            }
        };

        if let Ok(float) = string.value.parse::<f64>() {
            if let Ok(exponent) = Literal::get_value::<f64>(&literal.primitive) {
                let result = float.powf(*exponent);

                return Ok(PrimitiveString::get_literal(&result.to_string(), interval));
            }

            if let Ok(exponent) = Literal::get_value::<i64>(&literal.primitive) {
                let exponent = *exponent as f64;
                let result = float.powf(exponent);

                return Ok(PrimitiveString::get_literal(&result.to_string(), interval));
            }

            return Err(ErrorInfo {
                message: "usage: parameter must be of type float or int".to_owned(),
                interval,
            });
        }

        Err(ErrorInfo {
            message: "usage: need to have one parameter".to_owned(),
            interval,
        })
    }

    fn cos(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "cos()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => {
                let result = res.cos();

                Ok(PrimitiveString::get_literal(&result.to_string(), interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn floor(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "floor()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => {
                let result = res.floor();

                Ok(PrimitiveString::get_literal(&result.to_string(), interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn ceil(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "ceil()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => {
                let result = res.ceil();

                Ok(PrimitiveString::get_literal(&result.to_string(), interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn round(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "round()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => {
                let result = res.round();

                Ok(PrimitiveString::get_literal(&result.to_string(), interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn sin(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "sin()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => {
                let result = res.sin();

                Ok(PrimitiveString::get_literal(&result.to_string(), interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn sqrt(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "sqrt()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => {
                let result = res.sqrt();

                Ok(PrimitiveString::get_literal(&result.to_string(), interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn tan(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "tan()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => {
                let result = res.tan();

                Ok(PrimitiveString::get_literal(&result.to_string(), interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn to_int(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_int()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => Ok(PrimitiveInt::get_literal(res as i64, interval)),
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }

    fn to_float(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_float()", interval)?;

        match string.value.parse::<f64>() {
            Ok(res) => Ok(PrimitiveFloat::get_literal(res, interval)),
            Err(_) => Err(ErrorInfo {
                message: "string must be a number: ident.is_number() == true".to_owned(),
                interval,
            }),
        }
    }
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

    pub fn get_literal(string: &str, interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveString::new(string));

        Literal {
            content_type: "string".to_owned(),
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

        let mut result = PrimitiveObject::get_literal(&hashmap, Interval { column: 0, line: 0 });
        result.set_content_type("text");

        Message {
            content_type: result.content_type,
            content: result.primitive.to_json(),
        }
    }
}
