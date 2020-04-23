use crate::data::primitive::string::PrimitiveString;
use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_failure, *};
use crate::parser::operator::parse_operator;
use crate::parser::parse_comments::comment;
use crate::parser::parse_literal::parse_literal_expr;
use crate::parser::parse_var_types::parse_basic_expr;
use crate::parser::state_context::StateContext;
use crate::parser::state_context::StringState;
use crate::parser::tools::get_interval;
use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_while;
use nom::character::is_space;
use nom::{
    bytes::complete::tag,
    error::ParseError,
    multi::many_till,
    sequence::{delimited, preceded},
    *,
};

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
                    Interval::new_as_u32(0, 0),
                )),
            ))
        }
        None => {
            println!("[!] ERROR: MISSING CLOSE QUOTES");
            unimplemented!();
        }
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
