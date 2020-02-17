use crate::error_format::data::ErrorInfo;
use crate::interpreter::data::MemoryType;
use crate::interpreter::message::Message;
use crate::parser::ast::Interval;
use crate::parser::literal::Literal;
use crate::primitive::int::PrimitiveInt;
use crate::primitive::object::PrimitiveObject;
use crate::primitive::string::PrimitiveString;
use crate::primitive::tools::check_division_by_zero_f64;
use crate::primitive::tools::check_usage;
use crate::primitive::Right;
use crate::primitive::{Primitive, PrimitiveType};
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

        // type_of() -> Primitive<String>
        map.insert("type_of", (type_of as PrimitiveMethod, Right::Read));

        // to_string() -> Primitive<String>
        map.insert("to_string", (to_string as PrimitiveMethod, Right::Read));

        // abs() -> Primitive<Float>
        map.insert("abs", (abs as PrimitiveMethod, Right::Read));

        // cos() -> Primitive<Float>
        map.insert("cos", (cos as PrimitiveMethod, Right::Read));

        // pow(Primitive<Float>) -> Primitive<Float>
        map.insert("pow", (pow as PrimitiveMethod, Right::Read));

        // floor() -> Primitive<Float>
        map.insert("floor", (floor as PrimitiveMethod, Right::Read));

        // ceil() -> Primitive<Float>
        map.insert("ceil", (ceil as PrimitiveMethod, Right::Read));

        // round() -> Primitive<Float>
        map.insert("round", (round as PrimitiveMethod, Right::Read));

        // sin() -> Primitive<Float>
        map.insert("sin", (sin as PrimitiveMethod, Right::Read));

        // sqrt() -> Primitive<Float>
        map.insert("sqrt", (sqrt as PrimitiveMethod, Right::Read));

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveFloat {
    pub value: f64,
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn type_of(
    _float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "type_of()", interval)?;

    Ok(PrimitiveString::get_literal("string", "float", interval))
}

fn to_string(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "to_string()", interval)?;

    Ok(PrimitiveString::get_literal(
        "string",
        &float.to_string(),
        interval,
    ))
}

fn abs(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "abs()", interval)?;

    let result = float.value.abs();

    Ok(PrimitiveFloat::get_literal("float", result, interval))
}

fn cos(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "cos()", interval)?;

    let result = float.value.cos();

    Ok(PrimitiveFloat::get_literal("float", result, interval))
}

fn pow(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 1, "pow(Primitive<Float>)", interval)?;

    let literal = match args.get(0) {
        Some(res) => res,
        None => {
            return Err(ErrorInfo {
                message: "usage: need to have one parameter".to_owned(),
                interval,
            });
        }
    };

    match Literal::get_value::<f64>(&literal.primitive) {
        Ok(res) => {
            let result = float.value.powf(*res);

            Ok(PrimitiveFloat::get_literal("float", result, interval))
        }
        Err(_) => Err(ErrorInfo {
            message: "usage: parameter must be of type float".to_owned(),
            interval,
        }),
    }
}

fn floor(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "floor()", interval)?;

    let result = float.value.floor();

    Ok(PrimitiveFloat::get_literal("float", result, interval))
}

fn ceil(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "ceil()", interval)?;

    let result = float.value.ceil();

    Ok(PrimitiveFloat::get_literal("float", result, interval))
}

fn round(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "round()", interval)?;

    let result = float.value.round();

    Ok(PrimitiveFloat::get_literal("float", result, interval))
}

fn sin(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "sin()", interval)?;

    let result = float.value.sin();

    Ok(PrimitiveFloat::get_literal("float", result, interval))
}

fn sqrt(
    float: &mut PrimitiveFloat,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    check_usage(args, 0, "sqrt()", interval)?;

    let result = float.value.sqrt();

    Ok(PrimitiveFloat::get_literal("float", result, interval))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveFloat {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn get_literal(content_type: &str, float: f64, interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveFloat::new(float));

        Literal {
            content_type: content_type.to_owned(),
            primitive,
            interval,
        }
    }
}

impl Primitive for PrimitiveFloat {
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

        let result =
            PrimitiveObject::get_literal("text", &hashmap, Interval { column: 0, line: 0 });

        Message {
            content_type: result.content_type,
            content: result.primitive.to_json(),
        }
    }
}
