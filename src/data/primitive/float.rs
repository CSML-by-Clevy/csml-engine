use crate::data::literal::ContentType;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::int::PrimitiveInt;
use crate::data::primitive::object::PrimitiveObject;
use crate::data::primitive::string::PrimitiveString;
use crate::data::primitive::tools::check_division_by_zero_f64;
use crate::data::primitive::Right;
use crate::data::primitive::{Primitive, PrimitiveType};
use crate::data::{ast::Interval, message::Message, Literal};
use crate::error_format::ErrorInfo;
use lazy_static::*;
use std::cmp::Ordering;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert("is_number", (PrimitiveFloat::is_number as PrimitiveMethod, Right::Read));
        map.insert("type_of", (PrimitiveFloat::type_of as PrimitiveMethod, Right::Read));
        map.insert("to_string", (PrimitiveFloat::to_string as PrimitiveMethod, Right::Read));
        
        map.insert("abs", (PrimitiveFloat::abs as PrimitiveMethod, Right::Read));
        map.insert("cos", (PrimitiveFloat::cos as PrimitiveMethod, Right::Read));
        map.insert("ceil", (PrimitiveFloat::ceil as PrimitiveMethod, Right::Read));
        map.insert("floor", (PrimitiveFloat::floor as PrimitiveMethod, Right::Read));
        map.insert("pow", (PrimitiveFloat::pow as PrimitiveMethod, Right::Read));
        map.insert("round", (PrimitiveFloat::round as PrimitiveMethod, Right::Read));
        map.insert("sin", (PrimitiveFloat::sin as PrimitiveMethod, Right::Read));
        map.insert("sqrt", (PrimitiveFloat::sqrt as PrimitiveMethod, Right::Read));
        map.insert("tan", (PrimitiveFloat::tan as PrimitiveMethod, Right::Read));
        map.insert("to_int", (PrimitiveFloat::to_int as PrimitiveMethod, Right::Read));
        map.insert("to_float", (PrimitiveFloat::to_float as PrimitiveMethod, Right::Read));

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveFloat {
    pub value: f64,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveFloat {
    fn is_number(
        _float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveBoolean::get_literal(true, interval))
    }

    fn type_of(
        _float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveString::get_literal("float", interval))
    }

    fn to_string(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveString::get_literal(&float.to_string(), interval))
    }
}

impl PrimitiveFloat {
    fn abs(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "abs() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = float.value.abs();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn cos(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "cos() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = float.value.cos();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn ceil(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "ceil() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = float.value.ceil();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn floor(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "floor() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = float.value.floor();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn pow(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "pow(exponent: number) => float";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let exponent = match args.get(0) {
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                let exponent = Literal::get_value::<i64>(&exponent.primitive)?;

                *exponent as f64
            }
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveFloat => {
                let exponent = Literal::get_value::<f64>(&exponent.primitive)?;

                *exponent
            }
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveString => {
                let exponent = Literal::get_value::<String>(&exponent.primitive)?;

                match exponent.parse::<f64>() {
                    Ok(res) => res,
                    Err(_) => {
                        return Err(ErrorInfo::new(
                            "error: exponent must be a number".to_owned(),
                            interval,
                        ));
                    }
                }
            }
            _ => {
                return Err(ErrorInfo::new(
                    "error: exponent must be a number".to_owned(),
                    interval,
                ));
            }
        };

        let result = float.value.powf(exponent);

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn round(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "round() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = float.value.round();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn sin(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "sin() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = float.value.sin();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn sqrt(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "sqrt() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = float.value.sqrt();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn tan(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "tan() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = float.value.tan();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn to_int(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_int() => int";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveInt::get_literal(float.value as i64, interval))
    }

    fn to_float(
        float: &mut PrimitiveFloat,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_float() => float";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveFloat::get_literal(float.value, interval))
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveFloat {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn get_literal(float: f64, interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveFloat::new(float));

        Literal {
            content_type: "float".to_owned(),
            primitive,
            interval,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Primitive for PrimitiveFloat {
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

        Err(ErrorInfo {
            message: format!("unknown method '{}' for type Float", name),
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

    fn is_cmp(&self, other: &dyn Primitive) -> Option<Ordering> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            return self.value.partial_cmp(&other.value);
        }

        None
    }

    fn do_add(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value + other.value;

            return Ok(Box::new(PrimitiveFloat::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Add: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value - other.value;

            return Ok(Box::new(PrimitiveFloat::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Sub: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            check_division_by_zero_f64(self.value, other.value)?;

            let result = self.value / other.value;

            return Ok(Box::new(PrimitiveFloat::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Div: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value * other.value;

            return Ok(Box::new(PrimitiveFloat::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Mul: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value % other.value;

            return Ok(Box::new(PrimitiveFloat::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Rem: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_bitand(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value as i64 & other.value as i64;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] BitAnd: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_bitor(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value as i64 | other.value as i64;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

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
        PrimitiveType::PrimitiveFloat
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self.value)
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn as_bool(&self) -> bool {
        self.value.is_normal()
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
                content_type: "float".to_owned(),
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
}
