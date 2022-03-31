use crate::data::tokens::*;
use crate::error_format::{gen_nom_failure, ERROR_LEFT_BRACE, ERROR_RIGHT_BRACE};
use nom::{
    bytes::complete::tag,
    error::{ContextError, ParseError},
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_l_brace<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    match tag(L_BRACE)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, _err))) | Err(Err::Failure((input, _err))) => {
            Err(gen_nom_failure(input, ERROR_LEFT_BRACE))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

pub fn parse_r_brace<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    match tag(R_BRACE)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, _err))) | Err(Err::Failure((input, _err))) => {
            Err(gen_nom_failure(input, ERROR_RIGHT_BRACE))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}
