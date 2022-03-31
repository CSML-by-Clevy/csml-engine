use crate::data::tokens::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while},
    character::complete::multispace0,
    error::ParseError,
    multi::many0,
    sequence::delimited,
    IResult, *,
};

fn comment_single_line<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    let (s, _) = tag("//")(s)?;

    take_till(|ch| ch == '\n')(s)
}

fn comment_delimited<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    let (s, _) = tag(START_COMMENT)(s)?;
    let val: IResult<Span<'a>, Span<'a>, E> = take_until(END_COMMENT)(s);
    match val {
        Ok((s, _)) => tag(END_COMMENT)(s),
        // Error in comment_delimited is if '*/' is not found so the rest of the file is commented
        Err(Err::Error(_e)) | Err(Err::Failure(_e)) => Ok((Span::new(""), Span::new(""))),
        Err(Err::Incomplete(_)) => Ok((Span::new(""), Span::new(""))),
    }
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

fn ws<'a, F: 'a, O, E: ParseError<Span<'a>>>(
    inner: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
