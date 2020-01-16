use crate::parser::{
    ast::{Expr, RangeInterval},
    parse_comments::comment,
    parse_var_types::{parse_basic_expr},
    tokens::*,
    tools::get_interval,
};

use nom::{
    bytes::complete::tag,
    multi::separated_list,
    bytes::complete::take_till1,
    combinator::{cut, map, opt},
    error::{context, ParseError},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

fn parse_str<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    take_till1(|c: char| "\"".contains(c))(s)
}

fn string<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    context("invalid JSON key format",
      preceded(
        tag(DOUBLE_QUOTE),
        cut(terminated(
            parse_str,
            tag(DOUBLE_QUOTE)
    ))))(s)
}

fn key_value<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, (Span<'a>, Expr), E> {
    separated_pair(preceded(comment, string), cut(preceded(comment, tag(COLON))), parse_basic_expr)(s)
}

pub fn parse_object<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (object, _)) =
    preceded(tag(L_BRACE),
             terminated(
                 tuple((
                     map(
                         separated_list(preceded(comment, tag(COMMA)), key_value),
                         |tuple_vec| {
                             tuple_vec.into_iter().map(|(k, v)| (String::from(k.fragment), v)).collect()
                         }),
                     opt(preceded(comment, tag(COMMA)))
                 )),
                 preceded(comment, tag(R_BRACE)),
             ),
    )(s)?;
    let (s, end) = preceded(comment, get_interval)(s)?;

    Ok((s, Expr::MapExpr(object, RangeInterval{start, end})))
}
