use crate::data::literal::ContentType;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::float::PrimitiveFloat;
use crate::data::primitive::object::PrimitiveObject;
use crate::data::primitive::string::PrimitiveString;
use crate::data::primitive::tools::check_division_by_zero_i64;
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

type PrimitiveMethod =
    fn(int: &mut PrimitiveInt, args: &[Literal], interval: Interval) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert("is_number", (PrimitiveInt::is_number as PrimitiveMethod, Right::Read));
        map.insert("type_of", (PrimitiveInt::type_of as PrimitiveMethod, Right::Read));
        map.insert("to_string", (PrimitiveInt::to_string as PrimitiveMethod, Right::Read));

        map.insert("abs", (PrimitiveInt::abs as PrimitiveMethod, Right::Read));
        map.insert("cos", (PrimitiveInt::cos as PrimitiveMethod, Right::Read));
        map.insert("ceil", (PrimitiveInt::ceil as PrimitiveMethod, Right::Read));
        map.insert("floor", (PrimitiveInt::floor as PrimitiveMethod, Right::Read));
        map.insert("pow", (PrimitiveInt::pow as PrimitiveMethod, Right::Read));
        map.insert("round", (PrimitiveInt::round as PrimitiveMethod, Right::Read));
        map.insert("sin", (PrimitiveInt::sin as PrimitiveMethod, Right::Read));
        map.insert("sqrt", (PrimitiveInt::sqrt as PrimitiveMethod, Right::Read));
        map.insert("tan", (PrimitiveInt::tan as PrimitiveMethod, Right::Read));
        map.insert("to_int", (PrimitiveInt::to_int as PrimitiveMethod, Right::Read));
        map.insert("to_float", (PrimitiveInt::to_float as PrimitiveMethod, Right::Read));

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveInt {
    pub value: i64,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveInt {
    fn is_number(
        _int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: is_number()".to_owned(),
                interval,
            });
        }

        Ok(PrimitiveBoolean::get_literal(true, interval))
    }

    fn type_of(
        _int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: type_of()".to_owned(),
                interval,
            });
        }

        Ok(PrimitiveString::get_literal("int", interval))
    }

    fn to_string(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: to_string()".to_owned(),
                interval,
            });
        }

        Ok(PrimitiveString::get_literal(&int.to_string(), interval))
    }

    fn abs(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: abs()".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        let result = float.abs();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn cos(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: cos()".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        let result = float.cos();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }


    fn ceil(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: ceil()".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        let result = float.ceil();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn floor(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: floor()".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        let result = float.floor();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn pow(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 1 {
            return Err(ErrorInfo {
                message: "usage: pow(Primitive<Int || Float>)".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        match args.get(0) {
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                let exponent = Literal::get_value::<i64>(&exponent.primitive)?;
                let exponent = *exponent as f64;
                let result = float.powf(exponent);

                Ok(PrimitiveFloat::get_literal(result, interval))
            }
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveFloat => {
                let exponent = Literal::get_value::<f64>(&exponent.primitive)?;
                let result = float.powf(*exponent);

                Ok(PrimitiveFloat::get_literal(result, interval))
            }
            _ => {
                Err(ErrorInfo {
                    message: "usage: pow(Primitive<Int || Float>)".to_owned(),
                    interval,
                })
            }
        }
    }

    fn round(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: round()".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        let result = float.round();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn sin(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: sin()".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        let result = float.sin();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn sqrt(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: sqrt()".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        let result = float.sqrt();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn tan(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: tan()".to_owned(),
                interval,
            });
        }

        let float = int.value as f64;

        let result = float.tan();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn to_int(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: to_int()".to_owned(),
                interval,
            });
        }

        Ok(PrimitiveInt::get_literal(int.value, interval))
    }

    fn to_float(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        if args.len() != 0 {
            return Err(ErrorInfo {
                message: "usage: to_float()".to_owned(),
                interval,
            });
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

impl Primitive for PrimitiveInt {
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
            message: format!("unknown method '{}' for type Int", name),
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

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Add: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value - other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Sub: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            check_division_by_zero_i64(self.value, other.value)?;

            if self.value % other.value != 0 {
                let result = self.value as f64 / other.value as f64;

                return Ok(Box::new(PrimitiveFloat::new(result)));
            } else {
                let result = self.value / other.value;

                return Ok(Box::new(PrimitiveInt::new(result)));
            }
        }

        Err(ErrorInfo {
            message: "[!] Div: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value * other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Mul: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value % other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] Rem: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_bitand(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value & other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(ErrorInfo {
            message: "[!] BitAnd: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_bitor(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value | other.value;

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
        PrimitiveType::PrimitiveInt
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
}
