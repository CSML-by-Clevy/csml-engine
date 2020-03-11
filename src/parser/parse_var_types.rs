use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_failure, ERROR_RIGHT_BRACKET};
use crate::parser::{
    operator::parse_operator,
    parse_comments::comment,
    parse_functions::parse_functions,
    parse_idents::{parse_idents_as, parse_idents_assignation, parse_idents_utilisation},
    parse_literal::parse_literal_expr,
    parse_object::parse_object,
    parse_parenthesis::parse_r_parentheses,
    parse_path::parse_path,
    parse_string::parse_string,
    tools::*,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt},
    error::ParseError,
    multi::separated_list,
    sequence::{delimited, preceded, terminated, tuple},
    Err, IResult,
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
        Err(Err::Error((s, _err))) | Err(Err::Failure((s, _err))) => {
            Err(gen_nom_failure(s, ERROR_RIGHT_BRACKET))
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

fn parse_assignation_without_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = parse_idents_assignation(s)?;
    let (s, _) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, expr) = preceded(comment, parse_operator)(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Assign(
            Box::new(Expr::IdentExpr(name)),
            Box::new(expr),
        )),
    ))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_expr_list<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = preceded(
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
    )(s)?;
    let (s, end) = get_interval(s)?;

    Ok((s, Expr::VecExpr(vec, RangeInterval { start, end })))
}

pub fn parse_expr_array<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = preceded(
        tag(L_BRACKET),
        cut(terminated(
            tuple((
                separated_list(preceded(comment, tag(COMMA)), parse_operator),
                opt(preceded(comment, tag(COMMA))),
            )),
            preceded(comment, parse_r_bracket),
        )),
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

    let (s, expr) = parse_path(s, expr)?;

    parse_idents_as(s, expr)
}
