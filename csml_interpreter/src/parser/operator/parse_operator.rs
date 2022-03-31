use crate::data::{ast::*, tokens::*};
use crate::parser::operator::tools::and_operator;
use crate::parser::operator::tools::or_operator;
use crate::parser::operator::tools::parse_infix_operators;
use crate::parser::operator::tools::parse_item_operator;
use crate::parser::operator::tools::parse_not_operator;
use crate::parser::operator::tools::parse_term_operator;
use crate::parser::parse_comments::comment;
use crate::parser::parse_var_types::parse_basic_expr;
use nom::{
    branch::alt,
    error::{ContextError, ParseError},
    multi::{many0, many1},
    sequence::{preceded, tuple},
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_and<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, _) = preceded(comment, and_operator)(s)?;
    parse_infix_expr(s)
}

fn parse_infix_expr<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, expr1) = alt((parse_postfix_operator, parse_item))(s)?;
    let infix: IResult<Span<'a>, Infix, E> = preceded(comment, parse_infix_operators)(s);
    match infix {
        Ok((s, operator)) => {
            let (s, expr2) = alt((parse_postfix_operator, parse_item))(s)?;
            Ok((
                s,
                Expr::InfixExpr(operator, Box::new(expr1), Box::new(expr2)),
            ))
        }
        Err(_) => Ok((s, expr1)),
    }
}

fn parse_postfix_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, vec) = preceded(comment, many1(parse_not_operator))(s)?;
    let (s, expr) = parse_item(s)?;

    Ok((s, Expr::PostfixExpr(vec, Box::new(expr))))
}

fn parse_and_condition<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    // let (s, init) = parse_infix_expr(s)?;
    // fold_many0(parse_and, init.init, |acc, value: Expr| {
    //     Expr::InfixExpr(Infix::And, Box::new(acc), Box::new(value))
    // })(s)
    let (s, value) = parse_infix_expr(s)?;

    let (s, mut v) = many0(parse_and)(s)?;

    let value = v.drain(0..).fold(value, |acc, expr| {
        Expr::InfixExpr(Infix::And, Box::new(acc), Box::new(expr))
    });

    Ok((s, value))
}

fn parse_item<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, value) = parse_term(s)?;

    let (s, mut v) = many0(tuple((preceded(comment, parse_item_operator), parse_term)))(s)?;

    let value = v.drain(0..).fold(value, |acc, (infix, expr)| {
        Expr::InfixExpr(infix, Box::new(acc), Box::new(expr))
    });

    Ok((s, value))
}

fn parse_term<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, value) = parse_basic_expr(s)?;

    let (s, mut v) = many0(tuple((
        preceded(comment, parse_term_operator),
        parse_basic_expr,
    )))(s)?;

    let value = v.drain(0..).fold(value, |acc, (infix, expr)| {
        Expr::InfixExpr(infix, Box::new(acc), Box::new(expr))
    });

    Ok((s, value))
}

fn parse_or<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, _) = preceded(comment, or_operator)(s)?;
    parse_and_condition(s)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, value) = parse_and_condition(s)?;

    let (s, mut v) = many0(parse_or)(s)?;

    let value = v.drain(0..).fold(value, |acc, expr| {
        Expr::InfixExpr(Infix::Or, Box::new(acc), Box::new(expr))
    });

    Ok((s, value))
}
