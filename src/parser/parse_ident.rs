use crate::parser::{
    ast::*,
    parse_comments::comment,
    parse_var_types::{parse_as_variable, parse_var_expr},
    tokens::*,
    tools::get_interval,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    combinator::opt,
    error::ParseError,
    sequence::delimited,
    sequence::preceded,
    *,
};

fn parse_box_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Box<Expr>, E> {
    let (s, expr) = alt((parse_as_variable, parse_var_expr))(s)?;
    Ok((s, Box::new(expr)))
}

pub fn parse_string<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, String, E> {
    let (s, val) = take_till1(|c| is_valid_char(c))(s)?;

    // TODO: see if return String can be &str
    Ok((s, val.fragment.to_owned()))
}

pub fn parse_ident<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E> {
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, parse_string)(s)?;
    let (s, index) = opt(delimited(
        preceded(comment, tag(L_BRACKET)),
        parse_box_expr,
        preceded(comment, tag(R_BRACKET)),
    ))(s)?;

    Ok((s, forma_ident(var, index, position)))
}

pub fn is_valid_char(input: char) -> bool {
    input != '_' && !input.is_alphanumeric()
}

pub fn forma_ident(ident: String, index: Option<Box<Expr>>, position: Interval) -> Identifier {
    Expr::new_ident(ident, position, index)
}
