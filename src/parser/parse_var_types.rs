use crate::data::{ast::*, tokens::*};
use crate::parser::operator::parse_operator;
use crate::parser::parse_actions::parse_assignation_without_path;
use crate::parser::parse_idents::parse_idents_as;
use crate::parser::parse_idents::parse_idents_utilisation;
use crate::parser::parse_parenthesis::parse_r_parentheses;
use crate::parser::{
    parse_comments::comment, parse_functions::parse_functions, parse_literal::parse_literal_expr,
    parse_object::parse_object, parse_string::parse_string, tools::*,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt},
    error::{context, ParseError},
    multi::separated_list,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, *,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_r_bracket<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    match tag(R_BRACKET)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, err))) | Err(Err::Failure((input, err))) => {
            let err = E::from_error_kind(input, err);
            Err(Err::Failure(E::add_context(
                input,
                "RightBracketError",
                err,
            )))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

fn parse_condition_group<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    delimited(
        preceded(comment, tag(L_PAREN)),
        parse_operator,
        preceded(comment, parse_r_parentheses),
    )(s)
}

fn parse_idents_expr_utilisation<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, idents) = parse_idents_utilisation(s)?;
    Ok((s, Expr::IdentExpr(idents)))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_expr_list<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = context(
        "list",
        preceded(
            tag(L_PAREN),
            cut(terminated(
                tuple((
                    separated_list(
                        preceded(comment, tag(COMMA)),
                        alt((parse_assignation_without_path, parse_operator)),
                    ),
                    opt(preceded(comment, tag(COMMA))),
                )),
                preceded(comment, parse_r_parentheses),
            )),
        ),
    )(s)?;
    let (s, end) = get_interval(s)?;

    Ok((s, Expr::VecExpr(vec, RangeInterval { start, end })))
}

pub fn parse_expr_array<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = context(
        "array",
        preceded(
            tag(L_BRACKET),
            cut(terminated(
                tuple((
                    separated_list(preceded(comment, tag(COMMA)), parse_operator),
                    opt(preceded(comment, tag(COMMA))),
                )),
                preceded(comment, parse_r_bracket),
            )),
        ),
    )(s)?;
    let (s, end) = get_interval(s)?;

    Ok((s, Expr::VecExpr(vec, RangeInterval { start, end })))
}

pub fn parse_basic_expr<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, expr) = preceded(
        comment,
        alt((
            parse_condition_group,
            parse_object,
            parse_expr_array,
            parse_literal_expr,
            parse_functions,
            parse_string,
            parse_idents_expr_utilisation,
        )),
    )(s)?;

    parse_idents_as(s, expr)
}
