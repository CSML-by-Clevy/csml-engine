use crate::data::tokens::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    combinator::value,
    sequence::pair,
    sequence::delimited,
    sequence::tuple,
    character::complete::multispace0,
    bytes::complete::is_not,
    error::{ParseError},
    multi::many0,
    IResult, *,
};


fn comment_single_line<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
  value(
    s, // Output is thrown away.
    pair(tag("//"), is_not("\n\r"))
  )(s)
}

fn comment_delimited<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    value(
      s, // Output is thrown away.
      tuple((
        tag(START_COMMENT),
        take_until(END_COMMENT),
        tag(END_COMMENT)
      ))
    )(s)
}

fn all_comments<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    alt((comment_single_line, comment_delimited))(s)
}

pub fn comment<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    let (s, _) = sp(s)?;

    let (s, _) = match many0(ws(all_comments))(s) {
        Ok(val) => val,
        Err(Err::Error((s, _val))) | Err(Err::Failure((s, _val))) => return Ok((s, s)),
        Err(Err::Incomplete(i)) => return Err(Err::Incomplete(i)),
    };

    Ok((s, s))
}

fn sp<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| WHITE_SPACE.contains(c))(s)
}


fn ws<'a, F: 'a, O, E: ParseError<Span<'a>>>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
  where
  F: Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}