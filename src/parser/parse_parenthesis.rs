use crate::data::tokens::*;
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
        Err(Err::Error((input, err))) | Err(Err::Failure((input, err))) => {
            let err = E::from_error_kind(input, err);
            Err(Err::Failure(E::add_context(
                input,
                "list elem type ( ... ) not found",
                err,
            )))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

pub fn parse_r_parentheses<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    match tag(R_PAREN)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, err))) | Err(Err::Failure((input, err))) => {
            let err = E::from_error_kind(input, err);
            Err(Err::Failure(E::add_context(
                input,
                "Arguments inside parentheses bad format or ) missing",
                err,
            )))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}
