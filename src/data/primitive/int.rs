use crate::data::literal::ContentType;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::float::PrimitiveFloat;
use crate::data::primitive::object::PrimitiveObject;
use crate::data::primitive::string::PrimitiveString;
use crate::data::primitive::tools::check_division_by_zero_i64;
use crate::data::primitive::tools::check_usage;
use crate::data::primitive::Right;
use crate::data::primitive::{Primitive, PrimitiveType};
use crate::data::{ast::Interval, message::Message, Literal};
use crate::error_format::*;
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
        map.insert("pow", (PrimitiveInt::pow as PrimitiveMethod, Right::Read));
        map.insert(
            "floor",
            (PrimitiveInt::floor as PrimitiveMethod, Right::Read),
        );
        map.insert("ceil", (PrimitiveInt::ceil as PrimitiveMethod, Right::Read));
        map.insert(
            "round",
            (PrimitiveInt::round as PrimitiveMethod, Right::Read),
        );
        map.insert("sin", (PrimitiveInt::sin as PrimitiveMethod, Right::Read));
        map.insert("sqrt", (PrimitiveInt::sqrt as PrimitiveMethod, Right::Read));
        map.insert("tan", (PrimitiveInt::tan as PrimitiveMethod, Right::Read));
        map.insert(
            "is_number",
            (PrimitiveInt::is_number as PrimitiveMethod, Right::Read),
        );
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

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveInt {
    pub value: i64,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveInt {
    fn type_of(
        _int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "type_of()", interval)?;

        Ok(PrimitiveString::get_literal("int", interval))
    }

    fn to_string(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_string()", interval)?;

        Ok(PrimitiveString::get_literal(&int.to_string(), interval))
    }

    fn abs(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "abs()", interval)?;

        let result = int.value as f64;
        let result = result.abs();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn cos(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "cos()", interval)?;

        let result = int.value as f64;
        let result = result.cos();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn pow(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "pow(Primitive<Int || Float>)", interval)?;

        let literal = match args.get(0) {
            Some(res) => res,
            None => return Err(gen_error_info(interval, ERROR_NUMBER_POW.to_owned())),
        };

        if let Some(exponent) = Literal::get_value::<f64>(&literal.primitive) {
            let float = int.value as f64;
            let result = float.powf(*exponent);

            return Ok(PrimitiveFloat::get_literal(result, interval));
        }
        if let Some(exponent) = Literal::get_value::<i64>(&literal.primitive) {
            let float = int.value as f64;
            let exponent = *exponent as f64;

            let result = float.powf(exponent);
            let result = result as i64;

            return Ok(PrimitiveInt::get_literal(result, interval));
        }

        Err(gen_error_info(interval, ERROR_NUMBER_POW.to_owned()))
    }

    fn floor(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "floor()", interval)?;

        let result = int.value as f64;
        let result = result.floor();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn ceil(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "ceil()", interval)?;

        let result = int.value as f64;
        let result = result.ceil();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn round(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "round()", interval)?;

        let result = int.value as f64;
        let result = result.round();
        let result = result as i64;

        Ok(PrimitiveInt::get_literal(result, interval))
    }

    fn sin(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "sin()", interval)?;

        let result = int.value as f64;
        let result = result.sin();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn sqrt(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "sqrt()", interval)?;

        let result = int.value as f64;
        let result = result.sqrt();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn tan(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "tan()", interval)?;

        let result = int.value as f64;
        let result = result.tan();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn is_number(
        _int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "is_number()", interval)?;

        Ok(PrimitiveBoolean::get_literal(true, interval))
    }

    fn to_int(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_int()", interval)?;

        Ok(PrimitiveInt::get_literal(int.value, interval))
    }

    fn to_float(
        int: &mut PrimitiveInt,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_float()", interval)?;

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

        Err(gen_error_info(
            interval,
            format!("[{}] {}", name, ERROR_INT_UNKONWN_METHOD),
        ))
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

        Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_ADD.to_owned(),
        ))
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value - other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_SUB.to_owned(),
        ))
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

        Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_DIV.to_owned(),
        ))
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value * other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_MUL.to_owned(),
        ))
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value % other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_REM.to_owned(),
        ))
    }

    fn do_bitand(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value & other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_BITAND.to_owned(),
        ))
    }

    fn do_bitor(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let result = self.value | other.value;

            return Ok(Box::new(PrimitiveInt::new(result)));
        }

        Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_BITOR.to_owned(),
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
