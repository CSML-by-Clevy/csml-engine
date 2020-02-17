use crate::data::{ast::*, tokens::*};
use crate::parser::{
    expressions_evaluation::operator_precedence,
    parse_actions::parse_assignation,
    parse_comments::comment,
    parse_functions::parse_functions,
    parse_idents::{get_string, get_tag, parse_idents, parse_idents_expr},
    parse_literal::parse_literal_expr,
    parse_object::parse_object,
    parse_string::parse_string,
    tools::*,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt},
    error::{context, ParseError},
    multi::separated_list,
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub fn parse_expr_list<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = context(
        "list",
        preceded(
            tag(L_PAREN),
            cut(terminated(
                tuple((
                    separated_list(preceded(comment, tag(COMMA)), parse_var_expr),
                    opt(preceded(comment, tag(COMMA))),
                )),
                preceded(comment, parse_r_parentheses),
            )),
        ),
    )(s)?;
    let (s, end) = get_interval(s)?;
    Ok((s, Expr::VecExpr(vec, RangeInterval { start, end })))
}

pub fn parse_expr_array<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = context(
        "array",
        preceded(
            tag(L_BRACKET),
            cut(terminated(
                tuple((
                    separated_list(preceded(comment, tag(COMMA)), parse_basic_expr),
                    opt(preceded(comment, tag(COMMA))),
                )),
                preceded(comment, parse_r_bracket),
            )),
        ),
    )(s)?;
    let (s, end) = get_interval(s)?;
    Ok((s, Expr::VecExpr(vec, RangeInterval { start, end })))
}

pub fn parse_basic_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, expr) = preceded(
        comment,
        alt((
            parse_condition_group,
            parse_object,
            parse_expr_array,
            parse_literal_expr,
            parse_functions,
            parse_string,
            parse_idents_expr,
        )),
    )(s)?;

    parse_as_idents(s, expr)
}

pub fn parse_var_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, expr) = preceded(comment, alt((parse_assignation, operator_precedence)))(s)?;
    parse_as_idents(s, expr)
}

pub fn parse_as_idents<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
    expr: Expr,
) -> IResult<Span<'a>, Expr, E> {
    let arg: IResult<Span<'a>, String, E> = preceded(comment, get_string)(s);
    match arg {
        Err(_) => Ok((s, expr)),
        Ok((s2, tmp)) => {
            let as_idents: IResult<Span<'a>, Identifier, E> =
                preceded(get_tag(tmp, AS), parse_idents)(s2);

            if let Ok((s, name)) = as_idents {
                (Ok((s, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr))))))
            } else {
                Ok((s, expr))
            }
        }
    }
}
