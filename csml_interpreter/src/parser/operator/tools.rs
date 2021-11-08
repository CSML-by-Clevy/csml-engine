use crate::{
    data::{ast::*, tokens::*},
};
use nom::{branch::alt, bytes::complete::tag, error::ParseError, *};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_not_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Postfix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(NOT)(s)?;
    Ok((rest, Postfix::Not))
}

pub fn addition_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag(ADDITION)(s)?;
    Ok((s, Infix::Addition))
}

pub fn subtraction_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag(SUBTRACTION)(s)?;
    Ok((s, Infix::Subtraction))
}

pub fn and_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(AND)(s)?;
    Ok((rest, Infix::And))
}

pub fn or_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(OR)(s)?;
    Ok((rest, Infix::Or))
}

pub fn divide_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag(DIVIDE)(s)?;
    Ok((s, Infix::Divide))
}

pub fn multiply_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag(MULTIPLY)(s)?;
    Ok((s, Infix::Multiply))
}

pub fn remainder_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag(REMAINDER)(s)?;
    Ok((s, Infix::Remainder))
}

pub fn not_equal_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(NOT_EQUAL)(s)?;
    Ok((rest, Infix::NotEqual))
}

pub fn equal_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(EQUAL)(s)?;
    Ok((rest, Infix::Equal))
}

pub fn not_match<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(NOT_MATCH)(s)?;
    Ok((rest, Infix::NotMatch))
}

pub fn parse_match<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(MATCH)(s)?;
    Ok((rest, Infix::Match))
}

pub fn greater_than_equal_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(GREATER_THAN_EQUAL)(s)?;
    Ok((rest, Infix::GreaterThanEqual))
}

pub fn less_than_equal_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(LESS_THAN_EQUAL)(s)?;
    Ok((rest, Infix::LessThanEqual))
}

pub fn greater_than_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(GREATER_THAN)(s)?;
    Ok((rest, Infix::GreaterThan))
}

pub fn less_than_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(LESS_THAN)(s)?;
    Ok((rest, Infix::LessThan))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_item_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    alt((subtraction_operator, addition_operator))(s)
}

pub fn parse_term_operator<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    alt((divide_operator, multiply_operator, remainder_operator))(s)
}

pub fn parse_infix_operators<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Infix, E>
where
    E: ParseError<Span<'a>>,
{
    alt((
        not_equal_operator,
        not_match,
        parse_match,
        equal_operator,
        greater_than_equal_operator,
        less_than_equal_operator,
        greater_than_operator,
        less_than_operator,
    ))(s)
}
