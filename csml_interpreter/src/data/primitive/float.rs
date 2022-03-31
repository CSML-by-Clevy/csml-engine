use crate::data::primitive::tools::check_division_by_zero_f64;
use crate::data::{
    ast::Interval,
    error_info::ErrorInfo,
    literal,
    literal::ContentType,
    message::Message,
    position::Position,
    primitive::{
        Primitive, PrimitiveBoolean, PrimitiveInt, PrimitiveObject, PrimitiveString, PrimitiveType,
        Right,
    },
    Data, Literal, MessageData, MSG,
};
use crate::error_format::*;
use phf::phf_map;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::{collections::HashMap, sync::mpsc};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    float: &mut PrimitiveFloat,
    args: &HashMap<String, Literal>,
    additional_info: &Option<HashMap<String, Literal>>,
    data: &mut Data,
    interval: Interval,
) -> Result<Literal, ErrorInfo>;

const FUNCTIONS: phf::Map<&'static str, (PrimitiveMethod, Right)> = phf_map! {
    "is_number" => (PrimitiveFloat::is_number as PrimitiveMethod, Right::Read),
    "is_int" => (PrimitiveFloat::is_int as PrimitiveMethod, Right::Read),
    "is_float" => (PrimitiveFloat::is_float as PrimitiveMethod, Right::Read),
    "type_of" => (PrimitiveFloat::type_of as PrimitiveMethod, Right::Read),
    "is_error" => (PrimitiveFloat::is_error as PrimitiveMethod, Right::Read),
    "get_info" => (PrimitiveFloat::get_info as PrimitiveMethod, Right::Read),
    "to_string" => (PrimitiveFloat::to_string as PrimitiveMethod, Right::Read),

    "precision" => (PrimitiveFloat::precision as PrimitiveMethod, Right::Read),
    "abs" => (PrimitiveFloat::abs as PrimitiveMethod, Right::Read),
    "cos" => (PrimitiveFloat::cos as PrimitiveMethod, Right::Read),
    "ceil" => (PrimitiveFloat::ceil as PrimitiveMethod, Right::Read),
    "floor" => (PrimitiveFloat::floor as PrimitiveMethod, Right::Read),
    "pow" => (PrimitiveFloat::pow as PrimitiveMethod, Right::Read),
    "round" => (PrimitiveFloat::round as PrimitiveMethod, Right::Read),
    "sin" => (PrimitiveFloat::sin as PrimitiveMethod, Right::Read),
    "sqrt" => (PrimitiveFloat::sqrt as PrimitiveMethod, Right::Read),
    "tan" => (PrimitiveFloat::tan as PrimitiveMethod, Right::Read),
    "to_int" => (PrimitiveFloat::to_int as PrimitiveMethod, Right::Read),
    "to_float" => (PrimitiveFloat::to_float as PrimitiveMethod, Right::Read),
};
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveFloat {
    pub value: f64,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveFloat {
    fn is_number(
        _float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(true, interval))
    }

    fn is_int(
        _float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_int() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn is_float(
        _float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_float() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(true, interval))
    }

    fn type_of(
        _float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal("float", interval))
    }

    fn get_info(
        _float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        literal::get_info(args, additional_info, interval, data)
    }

    fn is_error(
        _float: &mut PrimitiveFloat,
        _args: &HashMap<String, Literal>,
        additional_info: &Option<HashMap<String, Literal>>,
        _data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        match additional_info {
            Some(map) if map.contains_key("error") => {
                Ok(PrimitiveBoolean::get_literal(true, interval))
            }
            _ => Ok(PrimitiveBoolean::get_literal(false, interval)),
        }
    }

    fn to_string(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal(&float.to_string(), interval))
    }
}

impl PrimitiveFloat {
    fn abs(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "abs() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let result = float.value.abs();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn cos(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "cos() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let result = float.value.cos();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn ceil(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "ceil() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let result = float.value.ceil();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn precision(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "precision(value) => float";

        let precision = match args.get("arg0") {
            Some(int) if int.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                Literal::get_value::<i64>(
                    &int.primitive,
                    &data.context.flow,
                    int.interval,
                    format!("usage: {}", usage),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval, &data.context.flow),
                    format!("usage: {}", usage),
                ))
            }
        };

        let result = format!("{:.*}", *precision as usize, float.value)
            .parse::<f64>()
            .unwrap_or(float.value);

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn floor(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "floor() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let result = float.value.floor();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn pow(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "pow(exponent: number) => float";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let exponent = match args.get("arg0") {
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                *Literal::get_value::<i64>(
                    &exponent.primitive,
                    &data.context.flow,
                    interval,
                    ERROR_NUMBER_POW.to_owned(),
                )? as f64
            }
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveFloat => {
                *Literal::get_value::<f64>(
                    &exponent.primitive,
                    &data.context.flow,
                    interval,
                    ERROR_NUMBER_POW.to_owned(),
                )?
            }
            Some(exponent) if exponent.primitive.get_type() == PrimitiveType::PrimitiveString => {
                let exponent = Literal::get_value::<String>(
                    &exponent.primitive,
                    &data.context.flow,
                    interval,
                    ERROR_NUMBER_POW.to_owned(),
                )?;

                match exponent.parse::<f64>() {
                    Ok(res) => res,
                    Err(_) => {
                        return Err(gen_error_info(
                            Position::new(interval, &data.context.flow),
                            ERROR_NUMBER_POW.to_owned(),
                        ));
                    }
                }
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval, &data.context.flow),
                    ERROR_NUMBER_POW.to_owned(),
                ));
            }
        };

        let result = float.value.powf(exponent);

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn round(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "round() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let result = float.value.round();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn sin(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "sin() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let result = float.value.sin();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn sqrt(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "sqrt() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let result = float.value.sqrt();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn tan(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "tan() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        let result = float.value.tan();

        Ok(PrimitiveFloat::get_literal(result, interval))
    }

    fn to_int(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_int() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveInt::get_literal(float.value as i64, interval))
    }

    fn to_float(
        float: &mut PrimitiveFloat,
        args: &HashMap<String, Literal>,
        _additional_info: &Option<HashMap<String, Literal>>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_float() => float";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
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
            additional_info: None,
            interval,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[typetag::serde]
impl Primitive for PrimitiveFloat {
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
            let lhs = self.value as i64;
            let rhs = other.value as i64;

            if lhs.checked_add(rhs).is_some() {
                return Ok(Box::new(PrimitiveFloat::new(self.value + other.value)));
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
            let lhs = self.value as i64;
            let rhs = other.value as i64;

            if lhs.checked_sub(rhs).is_some() {
                return Ok(Box::new(PrimitiveFloat::new(self.value - other.value)));
            }

            error_msg = OVERFLOWING_OPERATION;
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
            check_division_by_zero_f64(self.value, other.value)?;

            let lhs = self.value as i64;
            let rhs = other.value as i64;

            if lhs.checked_div(rhs).is_some() {
                return Ok(Box::new(PrimitiveFloat::new(self.value / other.value)));
            }

            error_msg = OVERFLOWING_OPERATION;
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
            let lhs = self.value as i64;
            let rhs = other.value as i64;

            if lhs.checked_mul(rhs).is_some() {
                return Ok(Box::new(PrimitiveFloat::new(self.value * other.value)));
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
            let lhs = self.value as i64;
            let rhs = other.value as i64;

            if lhs.checked_rem(rhs).is_some() {
                return Ok(Box::new(PrimitiveFloat::new(self.value % other.value)));
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
        PrimitiveType::PrimitiveFloat
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
                additional_info: None,
                interval: Interval {
                    start_column: 0,
                    start_line: 0,
                    offset: 0,
                    end_line: None,
                    end_column: None,
                },
            },
        );

        let mut result = PrimitiveObject::get_literal(
            &hashmap,
            Interval {
                start_column: 0,
                start_line: 0,
                offset: 0,
                end_line: None,
                end_column: None,
            },
        );
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
        additional_info: &Option<HashMap<String, Literal>>,
        interval: Interval,
        _content_type: &ContentType,
        data: &mut Data,
        _msg_data: &mut MessageData,
        _sender: &Option<mpsc::Sender<MSG>>,
    ) -> Result<(Literal, Right), ErrorInfo> {
        if let Some((f, right)) = FUNCTIONS.get(name) {
            let res = f(self, args, additional_info, data, interval)?;

            return Ok((res, *right));
        }

        Err(gen_error_info(
            Position::new(interval, &data.context.flow),
            format!("[{}] {}", name, ERROR_FLOAT_UNKNOWN_METHOD),
        ))
    }
}
