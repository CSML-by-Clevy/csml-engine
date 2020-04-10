use crate::data::{ast::Interval, tokens::*};
use nom::{
    bytes::complete::take_till1,
    error::{ErrorKind, ParseError},
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn position<'a, E: ParseError<Span<'a>>, T>(s: T) -> IResult<T, T, E>
where
    T: InputIter + InputTake,
    E: nom::error::ParseError<T>,
{
    nom::bytes::complete::take(0usize)(s)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_interval<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Interval, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, pos) = position(s)?;
    Ok((s, Interval::new_as_span(pos)))
}

pub fn get_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, val) = take_till1(|c: char| c != UNDERSCORE && !c.is_alphanumeric())(s)?;

    // TODO: see if return String can be &str ?
    Ok((s, (*val.fragment()).to_string()))
}

pub fn get_tag<I, E: ParseError<I>>(
    var: String,
    tag: &str,
) -> impl Fn(I) -> IResult<I, (), E> + '_ {
    move |input: I| {
        if var == tag {
            Ok((input, ()))
        } else {
            Err(Err::Error(E::from_error_kind(input, ErrorKind::Tag)))
        }
    }
}
