use crate::data::{ast::*, tokens::*};
use crate::parser::{parse_comments::comment, tools::get_interval};

use nom::{
    bytes::complete::tag,
    bytes::complete::take_till1,
    combinator::{cut, map, opt},
    error::{context, ParseError},
    multi::separated_list,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use crate::parser::parse_var_types::parse_basic_expr;
use crate::parser::state_context::StateContext;
use crate::parser::state_context::StringState;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_str<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    match StateContext::get_string() {
        StringState::Normal => take_till1(|c: char| "\"".contains(c))(s),
        StringState::Expand => take_till1(|c: char| "\\\"".contains(c))(s),
    }
}

fn string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    let token = match StateContext::get_string() {
        StringState::Normal => "\"",
        StringState::Expand => "\\\"",
    };

    context(
        "string must start with '\"' ",
        preceded(
            tag(token),
            cut(terminated(parse_str, tag(token))),
        ),
    )(s)
}

fn key_value<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Span<'a>, Expr), E>
where
    E: ParseError<Span<'a>>,
{
    separated_pair(
        preceded(comment, string),
        cut(preceded(comment, tag(COLON))),
        parse_basic_expr,
    )(s)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_object<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (object, _)) = preceded(
        tag(L_BRACE),
        terminated(
            tuple((
                map(
                    separated_list(preceded(comment, tag(COMMA)), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .map(|(k, v)| (String::from(*k.fragment()), v))
                            .collect()
                    },
                ),
                opt(preceded(comment, tag(COMMA))),
            )),
            preceded(comment, tag(R_BRACE)),
        ),
    )(s)?;
    let (s, end) = preceded(comment, get_interval)(s)?;

    Ok((s, Expr::MapExpr(object, RangeInterval { start, end })))
}
