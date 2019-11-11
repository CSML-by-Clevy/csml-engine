use crate::parser::{
    ast::*,
    expressions_evaluation::operator_precedence,
    parse_actions::{parse_actions, parse_assignation},
    parse_comments::comment,
    parse_ident::{parse_ident, get_tag, parse_ident_no_check},
    parse_literal::parse_literalexpr,
    parse_string::parse_string,
    tokens::*,
    tools::*,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    error::ParseError,
    multi::fold_many0,
    sequence::delimited,
    sequence::preceded,
    *,
};

fn parse_builderexpr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ident) = parse_identexpr(s)?;
    let (s, _) = preceded(comment, tag(DOT))(s)?;
    let (s, exp) = alt((parse_builderexpr, parse_identexpr))(s)?;
    Ok((s, Expr::BuilderExpr(Box::new(ident), Box::new(exp))))
}

fn parse_identexpr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    // let (s, ident) = parse_ident(s)?;
    let (s, ident) = parse_ident_no_check(s)?;
    Ok((s, Expr::IdentExpr(ident)))
}

fn pars_args<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, tag(COMMA))(s)?;
    alt((parse_as_variable, parse_var_expr))(s)
}

fn get_list<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, first_elem) = alt((parse_as_variable, parse_var_expr))(s)?;
    let (s, start) = get_interval(s)?;
    let (s, vec) = fold_many0(pars_args, vec![first_elem], |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(s)?;
    let (s, end) = get_interval(s)?;
    Ok((s, Expr::VecExpr(vec, RangeInterval { start, end })))
}

fn get_empty_list<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, tag(L_PAREN))(s)?;
    let (s, start) = get_interval(s)?;
    let (s, _) = preceded(comment, parse_r_parentheses)(s)?;
    let (s, end) = get_interval(s)?;
    Ok((s, Expr::VecExpr(vec![], RangeInterval { start, end })))
}

pub fn parse_expr_list<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    alt((
        delimited(
            preceded(comment, tag(L_PAREN)),
            get_list,
            preceded(comment, parse_r_parentheses),
        ),
        get_empty_list,
    ))(s)
}

fn get_empty_array<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, tag(L_BRACKET))(s)?;
    let (s, start) = get_interval(s)?;
    let (s, _) = preceded(comment, parse_r_bracket)(s)?;
    let (s, end) = get_interval(s)?;
    Ok((s, Expr::VecExpr(vec![], RangeInterval { start, end })))
}

pub fn parse_expr_array<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    alt((
        delimited(
            preceded(comment, tag(L_BRACKET)),
            get_list,
            preceded(comment, parse_r_bracket),
        ),
        get_empty_array,
    ))(s)
}

pub fn parse_mandatory_expr_list<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Expr, E> {
    delimited(
        preceded(comment, parse_l_parentheses),
        get_list,
        preceded(comment, parse_r_parentheses),
    )(s)
}

pub fn parse_basic_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    preceded(
        comment,
        alt((
            parse_literalexpr,
            parse_actions,
            parse_builderexpr,
            parse_string,
            parse_identexpr,
        )),
    )(s)
}

pub fn parse_var_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    preceded(
        comment,
        alt((
            parse_expr_array,
            parse_assignation,
            operator_precedence,
            parse_basic_expr,
        )),
    )(s)
}

pub fn parse_as_basic_variable<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Expr, E> {
    let (s, expr) = parse_basic_expr(s)?;
    let s = match get_tag(s, AS) {
        Err(Err::Error(err))
        | Err(Err::Failure(err)) => return Err(Err::Error(E::add_context(
            s,
            "msg for parse_as_basic_variable",
            err,
        ))),
        Err(Err::Incomplete(err)) => return Err(Err::Incomplete(err)),
        Ok((var, ..)) => var,
    };
    let (s, name) = parse_ident(s)?;
    (Ok((s, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr))))))
}

pub fn parse_as_variable<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, expr) = parse_var_expr(s)?;
    // TODO: get_ident ?
    let s = match get_tag(s, AS) {
        Err(Err::Error(err))
        | Err(Err::Failure(err)) => return Err(Err::Error(E::add_context(
            s,
            "msg for parse_as_basic_variable",
            err,
        ))),
        Err(Err::Incomplete(err)) => return Err(Err::Incomplete(err)),
        Ok((var, ..)) => var,
    };
    let (s, name) = parse_ident(s)?;
    (Ok((s, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr))))))
}
