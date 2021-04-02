use crate::data::position::Position;
use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_failure, ERROR_GOTO_STEP};
use crate::parser::{
    get_interval, parse_comments::comment, parse_idents::parse_string_assignation,
    tools::get_string, tools::get_tag, GotoType, GotoValueType, StateContext,
};

use nom::{branch::alt, bytes::complete::tag, combinator::opt, error::*, sequence::preceded, *};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_variable<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoValueType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, ..) = tag(DOLLAR)(s)?;
    let (s, name) = parse_string_assignation(s)?;
    Ok((s, GotoValueType::Variable(Expr::new_idents(name, position))))
}

fn get_name<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoValueType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, name) = parse_string_assignation(s)?;

    Ok((s, GotoValueType::Name(Expr::new_idents(name, position))))
}

fn get_goto_value_type<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoValueType, E>
where
    E: ParseError<Span<'a>>,
{
    alt((get_variable, get_name))(s)
}

fn get_step<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, STEP)(s)?;

    let (s, step) = preceded(comment, get_goto_value_type)(s)?;

    Ok((s, GotoType::Step(step)))
}

fn get_flow<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, FLOW)(s)?;

    let (s, flow) = preceded(comment, get_goto_value_type)(s)?;

    Ok((s, GotoType::Flow(flow)))
}

fn get_step_at_flow<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, ..) = comment(s)?;
    let (s, interval) = get_interval(s)?;

    let (s, step) = opt(get_goto_value_type)(s)?;
    let (s, at) = opt(tag("@"))(s)?;
    let (s, flow) = opt(get_goto_value_type)(s)?;

    let (step, flow) = match (step, at, flow) {
        (Some(step), Some(..), Some(flow)) => (step, flow),
        (None, Some(..), Some(flow)) => (
            GotoValueType::Name(Expr::new_idents("start".to_owned(), interval)),
            flow,
        ),
        (Some(step), Some(..), None) => (
            step,
            GotoValueType::Name(Expr::new_idents(Position::get_flow(), interval)),
        ),
        (Some(step), None, None) => (
            step,
            GotoValueType::Name(Expr::new_idents(Position::get_flow(), interval)),
        ),
        (None, Some(..), None) => (
            GotoValueType::Name(Expr::new_idents("start".to_owned(), interval)),
            GotoValueType::Name(Expr::new_idents(Position::get_flow(), interval)),
        ),
        _ => return Err(gen_nom_failure(s, ERROR_GOTO_STEP)),
    };

    Ok((s, GotoType::StepFlow { step, flow }))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_goto<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, GOTO)(s)?;

    let (s, interval) = get_interval(s)?;

    let (s, goto_type) = alt((get_step, get_flow, get_step_at_flow))(s)?;

    let instruction_info = InstructionInfo {
        index: StateContext::get_rip(),
        total: 0,
    };

    StateContext::inc_rip();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Goto(goto_type, interval)),
            instruction_info,
        ),
    ))
}
