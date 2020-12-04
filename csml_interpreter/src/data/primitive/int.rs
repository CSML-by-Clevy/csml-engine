use crate::data::error_info::ErrorInfo;
use crate::data::literal::ContentType;
use crate::data::position::Position;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::float::PrimitiveFloat;
use crate::data::primitive::object::PrimitiveObject;
use crate::data::primitive::string::PrimitiveString;
use crate::data::primitive::tools::check_division_by_zero_i64;
use crate::data::primitive::Right;
use crate::data::primitive::{Primitive, PrimitiveType};
use crate::data::{ast::Interval, message::Message, Literal};
use crate::error_format::*;
use lazy_static::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    int: &mut PrimitiveInt,
    args: &HashMap<String, Literal>,
    interval: Interval,
) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "is_number",
            (PrimitiveInt::is_number as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_int",
            (PrimitiveInt::is_int as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_float",
            (PrimitiveInt::is_float as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "type_of",
            (PrimitiveInt::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveInt::to_string as PrimitiveMethod, Right::Read),
        );

        map.insert("abs", (PrimitiveInt::abs as PrimitiveMethod, Right::Read));
        map.insert("cos", (PrimitiveInt::cos as PrimitiveMethod, Right::Read));
        map.insert("ceil", (PrimitiveInt::ceil as PrimitiveMethod, Right::Read));
        map.insert(
            "floor",
            (PrimitiveInt::floor as PrimitiveMethod, Right::Read),
        );
        map.insert("pow", (PrimitiveInt::pow as PrimitiveMethod, Right::Read));
        map.insert(
            "round",
            (PrimitiveInt::round as PrimitiveMethod, Right::Read),
        );
        map.insert("sin", (PrimitiveInt::sin as PrimitiveMethod, Right::Read));
        map.insert("sqrt", (PrimitiveInt::sqrt as PrimitiveMethod, Right::Read));
        map.insert("tan", (PrimitiveInt::tan as PrimitiveMethod, Right::Read));
        map.insert(
            "to_int",
            (PrimitiveInt::to_int as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_float",
            (PrimitiveInt::to_float as PrimitiveMethod, Right::Read),
        );

        map
    };
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveInt {
    pub value: i64,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveInt {
    fn is_number(
        _int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(true, interval))
    }

    fn is_int(
        _int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_int() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(true, interval))
    }

    fn is_float(
        _int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_float() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn type_of(
        _int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal("int", interval))
    }

    fn to_string(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal(&int.to_string(), interval))
    }
}

impl PrimitiveInt {
    fn abs(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "abs() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let result = float.abs();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn cos(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "cos() => number";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let result = float.cos();

        match result == (result as i64) as f64 {
            true => Ok(PrimitiveInt::get_literal(result as i64, interval)),
            false => Ok(PrimitiveFloat::get_literal(result, interval)),
        }
    }

    fn ceil(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "ceil() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let result = float.ceil();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn floor(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "floor() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let result = float.floor();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn pow(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "pow(exponent: number) => number";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let exponent = match args.get("arg0") {
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                *Literal::get_value::<i64>(
                    &exponent.primitive,
                    interval,
                    ERROR_NUMBER_POW.to_owned(),
                )? as f64
            }
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveFloat => {
                *Literal::get_value::<f64>(
                    &exponent.primitive,
                    interval,
                    ERROR_NUMBER_POW.to_owned(),
                )?
            }
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveString => {
                let exponent = Literal::get_value::<String>(
                    &exponent.primitive,
                    interval,
                    ERROR_NUMBER_POW.to_owned(),
                )?;

                match exponent.parse::<f64>() {
                    Ok(res) => res,
                    Err(_) => {
                        return Err(gen_error_info(
                            Position::new(interval),
                            ERROR_NUMBER_POW.to_owned(),
                        ));
                    }
                }
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_NUMBER_POW.to_owned(),
                ));
            }
        };

        let result = float.powf(exponent);

        match result == (result as i64) as f64 {
            true => Ok(PrimitiveInt::get_literal(result as i64, interval)),
            false => Ok(PrimitiveFloat::get_literal(result, interval)),
        }
    }

    fn round(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "round() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let result = float.round();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn sin(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "sin() => number";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let result = float.sin();

        match result == (result as i64) as f64 {
            true => Ok(PrimitiveInt::get_literal(result as i64, interval)),
            false => Ok(PrimitiveFloat::get_literal(result, interval)),
        }
    }

    fn sqrt(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "round() => number";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let result = float.sqrt();

        match result == (result as i64) as f64 {
            true => Ok(PrimitiveInt::get_literal(result as i64, interval)),
            false => Ok(PrimitiveFloat::get_literal(result, interval)),
        }
    }

    fn tan(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "tan() => number";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let float = int.value as f64;

        let result = float.tan();

        match result == (result as i64) as f64 {
            true => Ok(PrimitiveInt::get_literal(result as i64, interval)),
            false => Ok(PrimitiveFloat::get_literal(result, interval)),
        }
    }

    fn to_int(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_int() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveInt::get_literal(int.value, interval))
    }

    fn to_float(
        int: &mut PrimitiveInt,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_float() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveFloat::get_literal(int.value as f64, interval))
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveInt {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn get_literal(int: i64, interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveInt::new(int));

        Literal {
            content_type: "int".to_owned(),
            primitive,
            interval,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[typetag::serde]
impl Primitive for PrimitiveInt {
    fn is_eq(&self, other: &dyn Primitive) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            return self.value == other.value;
        }

        false
    }

    fn is_cmp(&self, other: &dyn Primitive) -> Option<Ordering> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            return self.value.partial_cmp(&other.value);
        }

        None
    }

    fn do_add(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let mut error_msg = ERROR_ILLEGAL_OPERATION;

        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            if let Some(value) = self.value.checked_add(other.value) {
                return Ok(Box::new(PrimitiveInt::new(value)));
            }

            error_msg = OVERFLOWING_OPERATION;
        }

        Err(format!(
            "{} {:?} + {:?}",
            error_msg,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let mut error_msg = ERROR_ILLEGAL_OPERATION;

        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            if let Some(value) = self.value.checked_sub(other.value) {
                return Ok(Box::new(PrimitiveInt::new(value)));
            }

            error_msg = OVERFLOWING_OPERATION
        }

        Err(format!(
            "{} {:?} - {:?}",
            error_msg,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let mut error_msg = ERROR_ILLEGAL_OPERATION;

        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            check_division_by_zero_i64(self.value, other.value)?;

            if self.value % other.value != 0 {
                if self.value.checked_div(other.value).is_some() {
                    let value = self.value as f64 / other.value as f64;

                    return Ok(Box::new(PrimitiveFloat::new(value)));
                }
            } else {
                if let Some(value) = self.value.checked_div(other.value) {
                    return Ok(Box::new(PrimitiveInt::new(value)));
                }

                error_msg = OVERFLOWING_OPERATION;
            }
        }

        Err(format!(
            "{} {:?} / {:?}",
            error_msg,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let mut error_msg = ERROR_ILLEGAL_OPERATION;

        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            if let Some(value) = self.value.checked_mul(other.value) {
                return Ok(Box::new(PrimitiveInt::new(value)));
            }

            error_msg = OVERFLOWING_OPERATION;
        }

        Err(format!(
            "{} {:?} * {:?}",
            error_msg,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        let mut error_msg = ERROR_ILLEGAL_OPERATION;

        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            if let Some(value) = self.value.checked_rem(other.value) {
                return Ok(Box::new(PrimitiveInt::new(value)));
            }

            error_msg = OVERFLOWING_OPERATION;
        }

        Err(format!(
            "{} {:?} % {:?}",
            error_msg,
            self.get_type(),
            other.get_type()
        ))
    }

    fn as_debug(&self) -> &dyn std::fmt::Debug {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::PrimitiveInt
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self.value)
    }

    fn format_mem(&self, _content_type: &str, _first: bool) -> serde_json::Value {
        serde_json::json!(self.value)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn as_bool(&self) -> bool {
        self.value.is_positive()
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
                content_type: "int".to_owned(),
                primitive: Box::new(PrimitiveString::new(&self.to_string())),
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

    fn do_exec(
        &mut self,
        name: &str,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &ContentType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        if let Some((f, right)) = FUNCTIONS.get(name) {
            let res = f(self, args, interval)?;

            return Ok((res, *right));
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", name, ERROR_INT_UNKNOWN_METHOD),
        ))
    }
}
