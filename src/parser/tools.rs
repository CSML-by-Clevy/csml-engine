use crate::data::{
    ast::{Expr, Interval},
    tokens::*,
};
use crate::parser::{expressions_evaluation::*, parse_comments::comment};
use nom::{
    bytes::complete::tag,
    error::ParseError,
    sequence::{delimited, preceded},
    *,
};

fn position<'a, E: ParseError<Span<'a>>, T>(s: T) -> IResult<T, T, E>
where
    T: InputIter + InputTake,
    E: nom::error::ParseError<T>,
{
    nom::bytes::complete::take(0usize)(s)
}

pub fn get_interval<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Interval, E> {
    let (s, pos) = position(s)?;
    Ok((s, Interval::new(pos)))
}

pub fn parse_l_parentheses<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Span<'a>, E> {
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

pub fn parse_r_parentheses<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Span<'a>, E> {
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

pub fn parse_strict_condition_group<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Expr, E> {
    delimited(
        preceded(comment, parse_l_parentheses),
        operator_precedence,
        preceded(comment, parse_r_parentheses),
    )(s)
}

pub fn parse_condition_group<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Expr, E> {
    delimited(
        preceded(comment, tag(L_PAREN)),
        operator_precedence,
        preceded(comment, parse_r_parentheses),
    )(s)
}

pub fn parse_r_bracket<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    match tag(R_BRACKET)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, err))) | Err(Err::Failure((input, err))) => {
            let err = E::from_error_kind(input, err);
            Err(Err::Failure(E::add_context(
                input,
                "RightBracketError",
                err,
            )))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

pub fn parse_l_brace<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    match tag(L_BRACE)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, err))) | Err(Err::Failure((input, err))) => {
            let err = E::from_error_kind(input, err);
            Err(Err::Failure(E::add_context(input, "LeftBraceError", err)))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

pub fn parse_r_brace<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    match tag(R_BRACE)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, err))) | Err(Err::Failure((input, err))) => {
            let err = E::from_error_kind(input, err);
            Err(Err::Failure(E::add_context(input, "RightBraceError", err)))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

pub fn parse_import_step<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Span<'a>, E> {
    match tag(STEP)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, err))) | Err(Err::Failure((input, err))) => {
            let err = E::from_error_kind(input, err);
            Err(Err::Failure(E::add_context(input, "ImportStepError", err)))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}
