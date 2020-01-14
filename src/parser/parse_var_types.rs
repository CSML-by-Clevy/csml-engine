use crate::parser::{
    ast::*,
    expressions_evaluation::operator_precedence,
    parse_actions::{parse_actions, parse_assignation},
    parse_comments::comment,
    parse_ident::{get_tag, parse_ident, parse_ident_no_check, get_string},
    parse_literal::{parse_literalexpr},
    parse_object::parse_object,
    parse_string::parse_string,
    tokens::*,
    tools::*,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt},
    error::{context, ParseError},
    multi::{separated_list, fold_many0},
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn get_builder_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    alt((parse_actions, parse_identexpr))(s)
}

fn parse_path<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ident) = get_builder_expr(s)?;
    let (s, _) = preceded(comment, tag(DOT))(s)?;
    Ok((s, ident))
}

fn get_builder_type<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, BuilderType, E> {
    let (s, ident) = parse_ident_no_check(s)?;
    let (s, _) = preceded(comment, tag(DOT))(s)?;

    // TODO: check for _metadata[n] | err ?
    if ident.ident == _METADATA {
        Ok((s, BuilderType::Metadata(ident.interval)))
    } else {
        Ok((s, BuilderType::Normal(ident)))
    }
}

fn parse_builderexpr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, builder_type) = get_builder_type(s)?;

    let (s, mut vec) = fold_many0(parse_path, vec![], |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(s)?;

    // TODO: last can be a fn
    let (s, last_elem) = get_builder_expr(s)?;
    vec.push(last_elem);
    Ok((s, Expr::BuilderExpr(builder_type, vec)))
}

fn parse_identexpr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ident) = parse_ident_no_check(s)?;
    Ok((s, Expr::IdentExpr(ident)))
}


pub fn parse_expr_list<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = context(
      "list",
      preceded(tag(L_PAREN),
      cut(terminated(
        tuple((
            separated_list(preceded(comment, tag(COMMA)), parse_var_expr),
            opt(preceded(comment, tag(COMMA)))
        )),
        preceded(comment, parse_r_parentheses)))
    ))(s)?;
    let (s, end) = get_interval(s)?;
    Ok((s, Expr::VecExpr(vec, RangeInterval { start, end })))
}

pub fn parse_expr_array<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (vec, _)) = context(
      "array",
      preceded(tag(L_BRACKET),
        cut(terminated(
            tuple((
                separated_list(preceded(comment, tag(COMMA)), parse_basic_expr),
                opt(preceded(comment, tag(COMMA)))
            )),
            preceded(comment, parse_r_bracket)))
    ))(s)?;
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
            parse_literalexpr,
            parse_actions,
            parse_builderexpr,
            parse_string,
            parse_identexpr,
        )),
    )(s)?;

    parse_as_ident(s, expr)
}

pub fn parse_var_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, expr) = preceded(
        comment,
        alt((
            parse_assignation,
            operator_precedence,
        )),
    )(s)?;
    parse_as_ident(s, expr)
}

pub fn parse_as_ident<'a, E: ParseError<Span<'a>>>(s: Span<'a>, expr: Expr) -> IResult<Span<'a>, Expr, E> {
    let arg: IResult<Span<'a>, String, E> = preceded(comment, get_string)(s);
    match arg {
        Err(_) => {
            Ok((s, expr))
        },
        Ok((s2, tmp)) => {
            let as_ident: IResult<Span<'a>, Identifier, E> = preceded(get_tag(tmp, AS), parse_ident)(s2);

            if let Ok((s, name)) = as_ident {
                (Ok((s, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr))))))
            } else {
                Ok((s, expr))
            }
        }
    }
}
