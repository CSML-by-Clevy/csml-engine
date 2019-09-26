use crate::error_format::data::ErrorInfo;
use crate::interpreter::variable_handler::match_literals::match_obj;
use crate::parser::{ast::Infix, literal::Literal};

pub fn evaluate(
    infix: &Infix,
    lit1: Result<Literal, ErrorInfo>,
    lit2: Result<Literal, ErrorInfo>,
) -> Result<Literal, ErrorInfo> {
    match (infix, lit1, lit2) {
        (Infix::NotEqual, Ok(l1), Ok(l2)) => Ok(Literal::boolean(l1 != l2, l1.get_interval())),
        (Infix::Equal, Ok(l1), Ok(l2)) => Ok(Literal::boolean(l1 == l2, l1.get_interval())),
        (Infix::GreaterThanEqual, Ok(l1), Ok(l2)) => {
            Ok(Literal::boolean(l1 >= l2, l1.get_interval()))
        }
        (Infix::LessThanEqual, Ok(l1), Ok(l2)) => Ok(Literal::boolean(l1 <= l2, l1.get_interval())),
        (Infix::GreaterThan, Ok(l1), Ok(l2)) => Ok(Literal::boolean(l1 > l2, l1.get_interval())),
        (Infix::LessThan, Ok(l1), Ok(l2)) => Ok(Literal::boolean(l1 < l2, l1.get_interval())),

        (Infix::Match, Ok(ref l1), Ok(ref l2)) => Ok(match_obj(&l1, &l2)),

        (Infix::Or, Ok(l1), Ok(l2)) => Ok(l1 | l2),
        (Infix::Or, Ok(l1), Err(_)) => Ok(l1.is_valid()),
        (Infix::Or, Err(_), Ok(l2)) => Ok(l2.is_valid()),
        (Infix::And, Ok(l1), Ok(l2)) => Ok(l1 & l2),

        (Infix::Adition, Ok(l1), Ok(l2)) => l1 + l2,
        (Infix::Substraction, Ok(l1), Ok(l2)) => l1 - l2,
        (Infix::Divide, Ok(l1), Ok(l2)) => l1 / l2,
        (Infix::Multiply, Ok(l1), Ok(l2)) => l1 * l2,
        (Infix::Remainder, Ok(l1), Ok(l2)) => l1 % l2,
        (_, Err(e), ..)
        | (_, .., Err(e)) => Err(e), //Ok(Literal::boolean(false, e.interval.to_owned())),
        (_, Ok(l1), ..) => Ok(Literal::boolean(false, l1.get_interval())),
    }
}
