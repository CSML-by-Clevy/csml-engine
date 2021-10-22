use crate::data::{ast::*, tokens::*, primitive::PrimitiveInt};
use crate::error_format::{gen_nom_failure, ERROR_RIGHT_BRACKET};
use crate::parser::{
    operator::{parse_operator, tools::parse_item_operator},
    parse_built_in::parse_built_in,
    parse_closure::parse_closure,
    parse_comments::comment,
    parse_idents::{parse_idents_as, parse_arg_idents_assignation, parse_idents_usage},
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

fn parse_condition_group<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;

    let (s, opt) = opt(preceded(comment, parse_item_operator))(s)?;
    let (s, expr) =  delimited(
            preceded(comment, tag(L_PAREN)),
            parse_operator,
            parse_r_parentheses,
    )(s)?;

    match opt {
        Some(infix) => {
            let zero = Expr::LitExpr{literal: PrimitiveInt::get_literal(0,interval), in_in_substring: false};
            Ok((s, Expr::InfixExpr(infix, Box::new(zero), Box::new(expr))))
        },
        None => Ok((s, expr))
    }
}

fn parse_assignation_without_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = parse_arg_idents_assignation(s)?;
    let (s, _) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, expr) = preceded(comment, parse_operator)(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Assign(
            AssignType::Assignment,
            Box::new(Expr::IdentExpr(name)),
            Box::new(expr),
        )),
    ))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_r_bracket<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
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

pub fn parse_idents_expr_usage<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, idents) = parse_idents_usage(s)?;

    Ok((s, Expr::IdentExpr(idents)))
}

pub fn parse_fn_args<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<String>, E>
where
    E: ParseError<Span<'a>>,
{
    let (start, _) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = parse_error(
        start,
        s,
        preceded(
            tag(L_PAREN),
            terminated(
                tuple((
                    separated_list(preceded(comment, tag(COMMA)), preceded(comment, get_string)),
                    opt(preceded(comment, tag(COMMA))),
                )),
                cut(parse_r_parentheses),
            ),
        ),
    )?;

    Ok((s, vec))
}

pub fn parse_expr_list<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (start, mut interval) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = parse_error(
        start,
        s,
        preceded(
            tag(L_PAREN),
            terminated(
                tuple((
                    separated_list(
                        preceded(comment, tag(COMMA)),
                        alt((parse_assignation_without_path, parse_operator)),
                    ),
                    opt(preceded(comment, tag(COMMA))),
                )),
                cut(parse_r_parentheses),
            ),
        ),
    )?;
    let (s, end) = get_interval(s)?;
    interval.add_end(end);

    Ok((s, Expr::VecExpr(vec, interval)))
}

pub fn parse_expr_array<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (start, mut interval) = preceded(comment, get_interval)(s)?;

    let (s, (vec, _)) = parse_error(
        start,
        s,
        preceded(
            tag(L_BRACKET),
            terminated(
                tuple((
                    separated_list(preceded(comment, tag(COMMA)), parse_operator), //parse_basic_expr
                    opt(preceded(comment, tag(COMMA))),
                )),
                preceded(comment, parse_r_bracket),
            ),
        ),
    )?;
    let (s, end) = get_interval(s)?;
    interval.add_end(end);

    Ok((s, Expr::VecExpr(vec, interval)))
}

pub fn parse_basic_expr<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, expr) = preceded(
        comment,
        alt((
            parse_closure,
            parse_condition_group,
            parse_object,
            parse_expr_array,
            parse_literal_expr,
            parse_built_in,
            parse_string,
            parse_idents_expr_usage,
        )),
    )(s)?;

    let (s, expr) = parse_path(s, expr)?;

    parse_idents_as(s, expr)
}
