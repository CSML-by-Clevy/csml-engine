use crate::error_format::{gen_error_info, ErrorInfo};
use crate::data::{
    ast::{Expr, Postfix, Infix},
    Literal, Data, MessageData, MSG,
    primitive::boolean::PrimitiveBoolean,
    position::Position,
};
use crate::interpreter::{
    variable_handler::{
        expr_to_literal, interval::interval_from_expr,
        match_literals::match_obj
    },
};
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn evaluate_infix(
    flow_name: &str,
    infix: &Infix,
    lhs: Result<Literal, ErrorInfo>,
    rhs: Result<Literal, ErrorInfo>,
) -> Result<Literal, ErrorInfo> {
    match (infix, lhs, rhs) {
        (Infix::Equal, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive == rhs.primitive,
            lhs.interval,
        )),
        (Infix::NotEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive != rhs.primitive,
            lhs.interval,
        )),
        (Infix::GreaterThanEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive >= rhs.primitive,
            lhs.interval,
        )),
        (Infix::LessThanEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive <= rhs.primitive,
            lhs.interval,
        )),
        (Infix::GreaterThan, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive > rhs.primitive,
            lhs.interval,
        )),
        (Infix::LessThan, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive < rhs.primitive,
            lhs.interval,
        )),

        (Infix::Addition, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive + rhs.primitive;
            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    interval: lhs.interval,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }
        (Infix::Subtraction, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive - rhs.primitive;

            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    interval: lhs.interval,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }
        (Infix::Divide, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive / rhs.primitive;

            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    interval: lhs.interval,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }

        (Infix::Multiply, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive * rhs.primitive;

            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    interval: lhs.interval,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }
        (Infix::Remainder, Ok(lhs), Ok(rhs)) => {
            let primitive = lhs.primitive % rhs.primitive;

            match primitive {
                Ok(primitive) => Ok(Literal {
                    content_type: primitive.get_type().to_string(),
                    primitive,
                    interval: lhs.interval,
                }),
                Err(err) => Err(gen_error_info(Position::new(lhs.interval, flow_name), err)),
            }
        }

        (Infix::Or, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive.as_bool() | rhs.primitive.as_bool(),
            lhs.interval,
        )),
        (Infix::And, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive.as_bool() & rhs.primitive.as_bool(),
            lhs.interval,
        )),
        (Infix::Match, Ok(ref lhs), Ok(ref rhs)) => Ok(PrimitiveBoolean::get_literal(
            match_obj(lhs, rhs),
            lhs.interval,
        )),
        (Infix::NotMatch, Ok(ref lhs), Ok(ref rhs)) => Ok(PrimitiveBoolean::get_literal(
            !match_obj(lhs, rhs),
            lhs.interval,
        )),
        (_, Err(e), ..) | (.., Err(e)) => Err(e),
    }
}

pub fn evaluate_postfix(
    postfixes: &[Postfix],
    expr: &Box<Expr>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
)  -> Result<Literal, ErrorInfo> {
    let value = valid_literal(expr_to_literal(expr, true, None, data, msg_data, sender));
    let interval = interval_from_expr(expr);

    if postfixes.len() % 2  == 0 {
        Ok(PrimitiveBoolean::get_literal(value, interval))
    } else {
        Ok(PrimitiveBoolean::get_literal(!value, interval))
    }
}

pub fn valid_literal(result: Result<Literal, ErrorInfo>) -> bool {
    match result {
        Ok(literal) => literal.primitive.as_bool(),
        Err(_) => false,
    }
}