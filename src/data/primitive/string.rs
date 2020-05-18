use crate::data::error_info::ErrorInfo;
use crate::data::literal::ContentType;
use crate::data::primitive::array::PrimitiveArray;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::float::PrimitiveFloat;
use crate::data::primitive::int::PrimitiveInt;
use crate::data::primitive::null::PrimitiveNull;
use crate::data::primitive::object::PrimitiveObject;
use crate::data::primitive::tools::*;
use crate::data::primitive::Right;
use crate::data::primitive::{Primitive, PrimitiveType};
use crate::data::{ast::Interval, message::Message, Literal};
use crate::error_format::*;
use lazy_static::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
// use serde_json::{Result, Value};
use crate::data::position::Position;
use crate::interpreter::json_to_literal;

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
            "is_number",
            (PrimitiveString::is_number as PrimitiveMethod, Right::Read),
        );
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
            (PrimitiveString::append as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "contains",
            (PrimitiveString::contains as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "contains_regex",
            (
                PrimitiveString::contains_regex as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "ends_with",
            (PrimitiveString::ends_with as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "ends_with_regex",
            (
                PrimitiveString::ends_with_regex as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "from_json",
            (PrimitiveString::from_json as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_empty",
            (PrimitiveString::is_empty as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "length",
            (PrimitiveString::length as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "match",
            (PrimitiveString::do_match as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "match_regex",
            (
                PrimitiveString::do_match_regex as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "starts_with",
            (PrimitiveString::starts_with as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "starts_with_regex",
            (
                PrimitiveString::starts_with_regex as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "to_lowercase",
            (
                PrimitiveString::to_lowercase as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "to_uppercase",
            (
                PrimitiveString::to_uppercase as PrimitiveMethod,
                Right::Read,
            ),
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
            "ceil",
            (PrimitiveString::ceil as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "floor",
            (PrimitiveString::floor as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "pow",
            (PrimitiveString::pow as PrimitiveMethod, Right::Read),
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
    fn is_number(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let result = string.value.parse::<f64>().is_ok();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn type_of(
        _string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal("string", interval))
    }

    fn to_string(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal(&string.to_string(), interval))
    }
}

impl PrimitiveString {
    fn append(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "append(value: string) => string";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut result = string.value.to_owned();

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_APPEND.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_APPEND.to_owned(),
                ));
            }
        };

        result.push_str(value);

        Ok(PrimitiveString::get_literal(&result, interval))
    }

    fn contains(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "contains(value: string) => boolean";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_DO_MATCH.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_DO_MATCH.to_owned(),
                ));
            }
        };

        let result = string.value.contains(value);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn contains_regex(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "contains_regex(value: string) => boolean";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_CONTAINS_REGEX.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_CONTAINS_REGEX.to_owned(),
                ));
            }
        };

        let action = match Regex::new(value) {
            Ok(res) => res,
            Err(_) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_CONTAINS_REGEX.to_owned(),
                ));
            }
        };

        let result = action.is_match(&string.value);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn ends_with(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "ends_with(value: string) => boolean";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_CONTAINS.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_END_WITH.to_owned(),
                ));
            }
        };

        let result = string.value.ends_with(value);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn ends_with_regex(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "ends_with_regex(value: string) => boolean";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_END_WITH_REGEX.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_END_WITH_REGEX.to_owned(),
                ));
            }
        };

        let action = match Regex::new(value) {
            Ok(res) => res,
            Err(_) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_END_WITH_REGEX.to_owned(),
                ));
            }
        };

        for key in action.find_iter(&string.value) {
            if key.end() == string.value.len() {
                return Ok(PrimitiveBoolean::get_literal(true, interval));
            }
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn from_json(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "from_json() => object";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let object = match serde_json::from_str(&string.value) {
            Ok(result) => result,
            Err(_) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_FROM_JSON.to_owned(),
                ));
            }
        };

        json_to_literal(&object, interval)
    }

    fn is_empty(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_empty() => boolean";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let result = string.value.is_empty();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn length(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "length() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let result = string.value.len();

        Ok(PrimitiveInt::get_literal(result as i64, interval))
    }

    fn do_match(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "match(value: string>) => array";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut vector: Vec<Literal> = Vec::new();

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_DO_MATCH.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_DO_MATCH.to_owned(),
                ));
            }
        };

        for result in string.value.matches(value) {
            vector.push(PrimitiveString::get_literal(result, interval));
        }

        if vector.is_empty() {
            return Ok(PrimitiveNull::get_literal(interval));
        }

        Ok(PrimitiveArray::get_literal(&vector, interval))
    }

    fn do_match_regex(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "match_regex(value: string>) => array";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut s: &str = &string.value;
        let mut vector: Vec<Literal> = Vec::new();

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_MATCH_REGEX.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_MATCH_REGEX.to_owned(),
                ));
            }
        };

        let action = match Regex::new(value) {
            Ok(res) => res,
            Err(_) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_VALID_REGEX.to_owned(),
                ));
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

    fn starts_with(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "starts_with(value: string) => boolean";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_START_WITH.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_START_WITH.to_owned(),
                ));
            }
        };

        let result = string.value.starts_with(value);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn starts_with_regex(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "starts_with_regex(value: string) => boolean";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let value = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_STRING_START_WITH_REGEX.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_START_WITH_REGEX.to_owned(),
                ));
            }
        };

        let action = match Regex::new(value) {
            Ok(res) => res,
            Err(_) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_STRING_VALID_REGEX.to_owned(),
                ));
            }
        };

        if let Some(res) = action.find(&string.value) {
            if res.start() == 0 {
                return Ok(PrimitiveBoolean::get_literal(true, interval));
            }
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn to_lowercase(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_lowercase() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut s = string.value.to_owned();

        s.make_ascii_lowercase();

        Ok(PrimitiveString::get_literal(&s, interval))
    }

    fn to_uppercase(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_uppercase() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut s = string.value.to_owned();

        s.make_ascii_uppercase();

        Ok(PrimitiveString::get_literal(&s, interval))
    }
}

impl PrimitiveString {
    fn abs(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("abs", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("abs", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "abs", ERROR_STRING_NUMERIC),
        ))
    }

    fn cos(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("cos", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("cos", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "cos", ERROR_STRING_NUMERIC),
        ))
    }

    fn ceil(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("ceil", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("ceil", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "ceil", ERROR_STRING_NUMERIC),
        ))
    }

    fn pow(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("pow", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("pow", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "pow", ERROR_STRING_NUMERIC),
        ))
    }

    fn floor(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("floor", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("floor", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "floor", ERROR_STRING_NUMERIC),
        ))
    }

    fn round(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("round", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("round", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "round", ERROR_STRING_NUMERIC),
        ))
    }

    fn sin(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("sin", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("sin", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "sin", ERROR_STRING_NUMERIC),
        ))
    }

    fn sqrt(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("sqrt", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("sqrt", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "sqrt", ERROR_STRING_NUMERIC),
        ))
    }

    fn tan(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("tan", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("tan", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "tan", ERROR_STRING_NUMERIC),
        ))
    }

    fn to_int(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("to_int", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("to_int", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "to_int", ERROR_STRING_NUMERIC),
        ))
    }

    fn to_float(
        string: &mut PrimitiveString,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if let Ok(int) = string.value.parse::<i64>() {
            let mut primitive = PrimitiveInt::new(int);

            let (literal, _right) =
                primitive.do_exec("to_float", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }
        if let Ok(float) = string.value.parse::<f64>() {
            let mut primitive = PrimitiveFloat::new(float);

            let (literal, _right) =
                primitive.do_exec("to_float", args, interval, &ContentType::Generics)?;

            return Ok(literal);
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", "to_float", ERROR_STRING_NUMERIC),
        ))
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
        _content_type: &ContentType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        if let Some((f, right)) = FUNCTIONS.get(name) {
            let res = f(self, args, interval)?;

            return Ok((res, *right));
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", name, ERROR_STRING_UNKNOWN_METHOD),
        ))
    }

    fn is_eq(&self, other: &dyn Primitive) -> bool {
        if let Some(rhs) = other.as_any().downcast_ref::<PrimitiveString>() {
            return match (get_integer(&self.value), get_integer(&rhs.value)) {
                (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => (lhs as f64) == rhs,
                (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => lhs == (rhs as f64),
                _ => self.value == rhs.value,
            };
        }

        false
    }

    fn is_cmp(&self, other: &dyn Primitive) -> Option<Ordering> {
        if let Some(rhs) = other.as_any().downcast_ref::<PrimitiveString>() {
            return match (get_integer(&self.value), get_integer(&rhs.value)) {
                (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => (lhs as f64).partial_cmp(&rhs),
                (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => lhs.partial_cmp(&(rhs as f64)),
                _ => self.value.partial_cmp(&rhs.value),
            };
        }

        None
    }

    fn do_add(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ERROR_STRING_RHS.to_owned());
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
            _ => Err(format!(
                "{} {:?} + {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ERROR_STRING_RHS.to_owned());
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
            _ => Err(format!(
                "{} {:?} - {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }

    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ERROR_STRING_RHS.to_owned());
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
            _ => Err(format!(
                "{} {:?} / {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ERROR_STRING_RHS.to_owned());
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
            _ => Err(format!(
                "{} {:?} * {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let rhs = match other.as_any().downcast_ref::<PrimitiveString>() {
            Some(res) => res,
            None => {
                return Err(ERROR_STRING_RHS.to_owned());
            }
        };

        match (get_integer(&self.value), get_integer(&rhs.value)) {
            (Ok(Integer::Int(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveInt::new(lhs % rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs % rhs)))
            }
            (Ok(Integer::Int(lhs)), Ok(Integer::Float(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs as f64 % rhs)))
            }
            (Ok(Integer::Float(lhs)), Ok(Integer::Int(rhs))) => {
                Ok(Box::new(PrimitiveFloat::new(lhs * rhs as f64)))
            }
            _ => Err(format!(
                "{} {:?} % {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
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
