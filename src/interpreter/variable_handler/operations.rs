use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::{ast::Infix, Literal};
use crate::error_format::{gen_error_info, ErrorInfo};
use crate::interpreter::variable_handler::match_literals::match_obj;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn evaluate(
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
                Err(err) => Err(gen_error_info(lhs.interval, err)),
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
                Err(err) => Err(gen_error_info(lhs.interval, err)),
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
                Err(err) => Err(gen_error_info(lhs.interval, err)),
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
                Err(err) => Err(gen_error_info(lhs.interval, err)),
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
                Err(err) => Err(gen_error_info(lhs.interval, err)),
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
        (Infix::Match, Ok(ref lhs), Ok(ref rhs)) => Ok(match_obj(lhs, rhs)),
        // TODO: [+] Handle Infix::NOT as Prefix !
        (Infix::Not, Ok(lhs), ..) => Ok(PrimitiveBoolean::get_literal(
            !lhs.primitive.as_bool(),
            lhs.interval,
        )),
        (_, Err(e), ..) | (.., Err(e)) => Err(e),
    }
}
