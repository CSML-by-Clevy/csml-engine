use crate::parser::{
    ast::*,
    parse_comments::comment,
    parse_var_types::parse_var_expr,
    tokens::*,
    tools::get_interval,
};
use nom::{
    bytes::complete::{tag, take_till1},
    combinator::opt,
    error::ErrorKind,
    error::ParseError,
    sequence::delimited,
    sequence::preceded,
    *,
};

fn parse_box_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Box<Expr>, E> {
    let (s, expr) = parse_var_expr(s)?;
    Ok((s, Box::new(expr)))
}

pub fn get_string<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, String, E> {
    let (s, val) = take_till1(|c| is_valid_char(c))(s)?;

    // TODO: see if return String can be &str ?
    Ok((s, val.fragment.to_owned()))
}

pub fn parse_ident_no_check<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Identifier, E> {
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;

    let (s, index) = opt(delimited(
        preceded(comment, tag(L_BRACKET)),
        parse_box_expr,
        preceded(comment, tag(R_BRACKET)),
    ))(s)?;

    Ok((s, forma_ident(var, index, position)))
}

pub fn parse_ident<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E> {
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;

    // TODO: change check to another fn ??
    if RESERVED.contains(&&(*var)) {
        return Err(Err::Failure(E::add_context(
            s,
            "reserved keyword can't be used as identifier",
            E::from_error_kind(s, ErrorKind::Tag),
        )));
    }
    let (s, index) = opt(delimited(
        preceded(comment, tag(L_BRACKET)),
        parse_box_expr,
        preceded(comment, tag(R_BRACKET)),
    ))(s)?;

    Ok((s, forma_ident(var, index, position)))
}

pub fn is_valid_char(input: char) -> bool {
    input != UNDERSCORE && !input.is_alphanumeric()
}

pub fn forma_ident(ident: String, index: Option<Box<Expr>>, position: Interval) -> Identifier {
    Expr::new_ident(ident, position, index)
}

pub fn get_tag<I, E: ParseError<I>>(
    var: String,
    tag: &str
) -> impl Fn(I) -> IResult<I, (), E> + '_ {
    move |input: I| {
        if var == tag {
            Ok((input, ()))
        } else {
            Err(Err::Error(E::from_error_kind(input, ErrorKind::Tag)))
        }
    }
}
