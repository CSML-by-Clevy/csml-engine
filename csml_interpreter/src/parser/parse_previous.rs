use crate::data::{ast::*, tokens::*};
use crate::parser::{
    get_interval, parse_comments::comment, 
    tools::get_string, tools::get_tag
};

use nom::{error::*, sequence::preceded, *};

fn get_previous<I, E: ParseError<I>>(
    var: String,
    interval: Interval
) -> impl FnMut(I) -> IResult<I, PreviousType, E> {
    move |input: I| {
        if var == FLOW {
            Ok((input, PreviousType::Flow(interval)))
        } else if var == STEP {
            Ok((input, PreviousType::Step(interval)))
        } else {
            Err(Err::Error(E::from_error_kind(input, ErrorKind::Tag)))
        }
    }
}

pub fn parse_previous<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, interval) = preceded(comment, get_interval)(s)?;
    let (s, name) = get_string(s)?;
    let (s, ..) = get_tag(name, PREVIOUS)(s)?;


    let (s, inter) = preceded(comment, get_interval)(s)?;
    let (s, name) = get_string(s)?;
    let (s, previous) = get_previous(name, inter)(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Previous(previous, interval)),
    ))
}