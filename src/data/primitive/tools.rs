use crate::data::{ast::Interval, Literal};
use crate::error_format::ErrorInfo;

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

pub fn get_integer(text: &str) -> Result<Integer, ErrorInfo> {
    match (text.parse::<i64>(), text.parse::<f64>()) {
        (Ok(int), _) => Ok(Integer::Int(int)),
        (_, Ok(float)) => Ok(Integer::Float(float)),
        (..) => Err(ErrorInfo {
            message: "[!] Ops: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        }),
    }
}

pub fn check_division_by_zero_i64(lhs: i64, rhs: i64) -> Result<i64, ErrorInfo> {
    if lhs == 0 || rhs == 0 {
        return Err(ErrorInfo {
            message: "[!] Int: Division by zero".to_owned(),
            interval: Interval { column: 0, line: 0 },
        });
    }

    Ok(lhs)
}

pub fn check_division_by_zero_f64(lhs: f64, rhs: f64) -> Result<f64, ErrorInfo> {
    if lhs == 0.0 || rhs == 0.0 {
        return Err(ErrorInfo {
            message: "[!] Float: Division by zero".to_owned(),
            interval: Interval { column: 0, line: 0 },
        });
    }

    Ok(lhs)
}

pub fn check_usage(
    args: &[Literal],
    len: usize,
    message: &str,
    interval: Interval,
) -> Result<u8, ErrorInfo> {
    if args.len() == len {
        return Ok(0);
    }

    Err(ErrorInfo {
        message: format!("usage: {}", message),
        interval,
    })
}
