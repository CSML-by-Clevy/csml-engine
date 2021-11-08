use crate::data::{
    ast::{Expr},
    tokens::{Span, WHILE, L_PAREN, R_PAREN},
};
use crate::parser::operator::parse_operator;
use crate::parser::{
    parse_comments::comment,
    parse_scope::parse_scope,
    tools::{get_interval},
};
use nom::{
    bytes::complete::tag,
    combinator::{cut},
    error::ParseError,
    sequence::preceded,
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_while<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag(WHILE))(s)?;
    let (s, mut interval) = get_interval(s)?;

    let (s, _) = cut(preceded(comment, tag(L_PAREN)))(s)?;
    let (s, expr) = cut(parse_operator)(s)?;
    let (s, _) = cut(preceded(comment, tag(R_PAREN)))(s)?;

    let (s, block) = parse_scope(s)?;
    let (s, end) = get_interval(s)?;
    interval.add_end(end);

    Ok((
        s,
        Expr::WhileExpr(Box::new(expr), block, interval),
    ))
}
