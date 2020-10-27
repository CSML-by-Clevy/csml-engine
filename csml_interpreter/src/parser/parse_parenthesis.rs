use crate::data::tokens::*;
use crate::error_format::{gen_nom_failure, ERROR_PARENTHESES, ERROR_PARENTHESES_END};
use crate::parser::parse_comments::comment;

use nom::{bytes::complete::tag, error::ParseError, *};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_l_parentheses<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    match tag(L_PAREN)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((s, _err))) | Err(Err::Failure((s, _err))) => {
            Err(gen_nom_failure(s, ERROR_PARENTHESES))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

pub fn parse_r_parentheses<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s2, _) = comment(s)?;
    match tag(R_PAREN)(s2) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((_, _err))) | Err(Err::Failure((_, _err))) => {
            Err(gen_nom_failure(s, ERROR_PARENTHESES_END))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}
