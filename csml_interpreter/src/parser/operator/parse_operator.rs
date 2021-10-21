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
    error::ParseError,
    multi::{fold_many0, many1},
    sequence::{preceded, tuple},
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_and<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, and_operator)(s)?;
    parse_infix_expr(s)
}

fn parse_infix_expr<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
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
    E: ParseError<Span<'a>>,
{
    let (s, vec) = preceded(comment, many1(parse_not_operator))(s)?;
    let (s, expr) = parse_item(s)?;

    Ok((
        s,
        // TODO: InfixExpr clone in not operator or create a new expr for not??
        Expr::PostfixExpr(vec, Box::new(expr)),
    ))
}

fn parse_and_condition<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, init) = parse_infix_expr(s)?;
    fold_many0(parse_and, init, |acc, value: Expr| {
        Expr::InfixExpr(Infix::And, Box::new(acc), Box::new(value))
    })(s)
}

fn parse_item<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, init) = parse_term(s)?;
    fold_many0(
        tuple((preceded(comment, parse_item_operator), parse_term)),
        init,
        |acc, v: (Infix, Expr)| Expr::InfixExpr(v.0, Box::new(acc), Box::new(v.1)),
    )(s)
}

fn parse_term<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, init) = parse_basic_expr(s)?;
    fold_many0(
        tuple((preceded(comment, parse_term_operator), parse_basic_expr)),
        init,
        |acc, v: (Infix, Expr)| Expr::InfixExpr(v.0, Box::new(acc), Box::new(v.1)),
    )(s)
}

fn parse_or<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, or_operator)(s)?;
    parse_and_condition(s)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, init) = parse_and_condition(s)?;
    fold_many0(parse_or, init, |acc, value: Expr| {
        Expr::InfixExpr(Infix::Or, Box::new(acc), Box::new(value))
    })(s)
}
