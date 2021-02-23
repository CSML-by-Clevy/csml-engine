use crate::data::primitive::string::PrimitiveString;
use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_failure, *};
use nom::{bytes::complete::tag, error::ParseError, sequence::delimited, *};

////////////////////////////////////////////////////////////////////////////////
/// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn do_parse_expand_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    match s.find_substring("\\\"") {
        Some(distance) => {
            let (rest, string) = s.take_split(distance);

            Ok((
                rest,
                Expr::LitExpr(PrimitiveString::get_literal(
                    string.fragment(),
                    Interval::new_as_u32(0, 0, 0, None, None),
                )),
            ))
        }
        None => Err(gen_nom_failure(s, ERROR_DOUBLE_QUOTE)),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_expand_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    delimited(tag("\\\""), do_parse_expand_string, tag("\\\""))(s)
}
