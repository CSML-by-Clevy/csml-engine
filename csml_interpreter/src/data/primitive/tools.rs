use crate::data::primitive::{PrimitiveString, PrimitiveType};
use crate::data::{Literal, Position};
use crate::error_format::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

pub enum Integer {
    Int(i64),
    Float(f64),
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_integer(text: &str) -> Result<Integer, String> {
    match (text.parse::<i64>(), text.parse::<f64>()) {
        (Ok(int), _) => Ok(Integer::Int(int)),
        (_, Ok(float)) => Ok(Integer::Float(float)),
        (..) => Err(ERROR_OPS.to_owned()),
    }
}

pub fn get_array(literal: Literal, flow_name: &str, error_message: String) -> Result<Vec<Literal>, ErrorInfo> {
    match literal.primitive.get_type() {
        PrimitiveType::PrimitiveString => {
            let string = Literal::get_value::<String>(
                &literal.primitive,
                flow_name,
                literal.interval.to_owned(),
                error_message,
            )?;

            Ok(PrimitiveString::get_array_char(
                string.to_owned(),
                literal.interval,
            ))
        }
        PrimitiveType::PrimitiveArray => Ok(Literal::get_value::<Vec<Literal>>(
            &literal.primitive,
            flow_name,
            literal.interval.to_owned(),
            error_message,
        )?
        .to_owned()),
        _ => Err(gen_error_info(
            Position::new(literal.interval, flow_name),
            error_message,
        )),
    }
}

pub fn check_division_by_zero_i64(lhs: i64, rhs: i64) -> Result<i64, String> {
    if lhs == 0 || rhs == 0 {
        return Err(ERROR_OPS_DIV_INT.to_owned());
    }

    Ok(lhs)
}

pub fn check_division_by_zero_f64(lhs: f64, rhs: f64) -> Result<f64, String> {
    if lhs == 0.0 || rhs == 0.0 {
        return Err(ERROR_OPS_DIV_FLOAT.to_owned());
    }

    Ok(lhs)
}
