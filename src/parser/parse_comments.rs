use crate::parser::tokens::Span;
use crate::parser::tokens::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    combinator::opt,
    error::ParseError,
    multi::many0,
    IResult, *,
};

fn comment_single_line<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    let (s, _) = tag(INLINE_COMMENT)(s)?;

    let val: IResult<Span<'a>, Span<'a>, E> = take_until("\n")(s);
    let val2: IResult<Span<'a>, Span<'a>, E> = take_until("\r\n")(s);
    if let Ok((s, v)) = val {
        Ok((s, v))
    } else if let Ok((s, v)) = val2 {
        Ok((s, v))
    } else {
        // if new line is not found the rest of the file is commented
        Ok((Span::new(""), Span::new("")))
    }
}

fn comment_delimited<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    let (s, _) = tag(START_COMMENT)(s)?;
    let val: IResult<Span<'a>, Span<'a>, E> = take_until(END_COMMENT)(s);
    match val {
        Ok((s, _)) => tag(END_COMMENT)(s),
        // Error in comment_delimited is if '*/' is not found so all the rest of the file is commented
        Err(Err::Error(_e)) | Err(Err::Failure(_e)) => Ok((Span::new(""), Span::new(""))),
        Err(Err::Incomplete(_)) => unimplemented!(),
    }
}

fn all_comments<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    let (s, _) = opt(sp)(s)?;
    alt((comment_delimited, comment_single_line))(s)
}

pub fn comment<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    let val = many0(all_comments)(s);
    let (s, _) = match val {
        Ok(val) => val,
        Err(Err::Error((s, _val))) | Err(Err::Failure((s, _val))) => return Ok((s.clone(), s)),
        Err(Err::Incomplete(i)) => return Err(Err::Incomplete(i)),
    };
    // Ok((s, s))
    sp(s)
}

fn sp<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| WHITE_SPACE.contains(c))(s)
}
