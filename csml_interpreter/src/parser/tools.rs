use crate::data::{ast::*, tokens::*};
use nom::{
    bytes::complete::take_while1,
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

fn set_escape(s: &str, index: usize, escape: &mut bool) {
    if let Some(c) = s.chars().nth(index) {
        if c == '\\' {
            return match escape {
                true => {
                    *escape = false;
                }
                false => {
                    *escape = true;
                }
            };
        }

        *escape = false;
    }
}

fn set_substring(s: &str, index: usize, escape: bool, expand: bool, substring: &mut bool) {
    if let Some(c) = s.chars().nth(index) {
        if c == '"' && escape && expand {
            match substring {
                true => {
                    *substring = false;
                }
                false => {
                    *substring = true;
                }
            }
        }
    }
}

fn set_open_expand(s: &str, index: usize, escape: bool, substring: bool, expand: &mut bool) {
    if let Some(c) = s.chars().nth(index) {
        if c == '{' && !escape && !substring {
            if let Some(c) = s.chars().nth(index + 1) {
                if c == '{' && !escape {
                    *expand = true;
                }
            }
        }
    }
}

fn set_close_expand(s: &str, index: usize, escape: bool, substring: bool, expand: &mut bool) {
    if let Some(c) = s.chars().nth(index) {
        if c == '}' && !escape && !substring {
            if let Some(c) = s.chars().nth(index + 1) {
                if c == '}' && !escape {
                    *expand = false;
                }
            }
        }
    }
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

pub fn get_range_interval(vector_interval: &[Interval]) -> Interval {
    let mut start = Interval::new_as_u32(0, 0, 0, None, None);
    let mut end = Interval::new_as_u32(0, 0, 0, None, None);

    for (index, interval) in vector_interval.iter().enumerate() {
        if index == 0 {
            start = *interval;
        }

        end = *interval;
    }

    start.add_end(end);
    start
}

// generate range error
pub fn parse_error<'a, O, E, F>(start: Span<'a>, span: Span<'a>, func: F ) -> IResult<Span<'a>, O, E>
where
    E: ParseError<Span<'a>>,
    F: Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
{
    match func(span) {
        Ok(value)=> Ok(value),
        Err(Err::Error(e)) => Err(Err::Error(e)),
        Err(Err::Failure(e)) => {
            return Err(Err::Failure(E::append(start, ErrorKind::Tag, e)))
        }
        Err(Err::Incomplete(needed)) => {
            return Err(Err::Incomplete(needed))
        },
    }
}

pub fn get_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, string) = take_while1(|c: char| c == '_' || c == '\\' || c.is_alphanumeric())(s)?;
    // let (rest, string) = take_till1(|c: char| c != UNDERSCORE && !c.is_alphanumeric())(s)?;

    // TODO: see if return can be &str ?
    Ok((rest, (*string.fragment()).to_string()))
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

pub fn get_distance_brace(s: &Span, key: char) -> Option<usize> {
    let mut escape: bool = false;
    let mut expand: bool = false;
    let mut substring: bool = false;

    for (i, c) in s.as_bytes().iter().enumerate() {
        if *c as char == key && !escape && !substring {
            if let Some(c) = s.as_bytes().iter().nth(i + 1) {
                if *c as char == key {
                    return Some(i);
                }
            }
        }

        set_open_expand(s.fragment(), i, escape, substring, &mut expand);
        set_close_expand(s.fragment(), i, escape, substring, &mut expand);
        set_substring(s.fragment(), i, escape, expand, &mut substring);
        set_escape(s.fragment(), i, &mut escape);
    }

    None
}
