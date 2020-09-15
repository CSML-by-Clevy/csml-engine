use crate::data::position::Position;
use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_failure, ERROR_GOTO_STEP};
use crate::linter::data::Linter;
use crate::parser::{
    get_interval, parse_comments::comment, parse_idents::parse_string_assignation,
    tools::get_string, tools::get_tag, GotoType, StateContext,
};

use nom::{branch::alt, bytes::complete::tag, combinator::opt, error::*, sequence::preceded, *};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_step<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, interval) = get_interval(s)?;
    let (s, ..) = get_tag(name, STEP)(s)?;

    let (s, step) = preceded(comment, parse_string_assignation)(s)?;
    let flow = Position::get_flow();

    Linter::add_goto(
        &Position::get_flow(),
        &Position::get_step(),
        &flow,
        &step,
        interval,
    );

    Ok((s, GotoType::Step(step)))
}

fn get_flow<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, interval) = get_interval(s)?;
    let (s, ..) = get_tag(name, FLOW)(s)?;

    let (s, flow) = preceded(comment, parse_string_assignation)(s)?;

    Linter::add_goto(
        &Position::get_flow(),
        &Position::get_step(),
        &flow,
        "start",
        interval,
    );

    Ok((s, GotoType::Flow(flow)))
}

fn get_step_at_flow<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, ..) = comment(s)?;
    let (s, interval) = get_interval(s)?;

    let (s, step) = opt(parse_string_assignation)(s)?;
    let (s, at) = opt(tag("@"))(s)?;
    let (s, flow) = opt(parse_string_assignation)(s)?;

    let (step, flow) = match (step, at, flow) {
        (Some(step), Some(..), Some(flow)) => (step, flow),
        (None, Some(..), Some(flow)) => ("start".to_owned(), flow),
        (Some(step), Some(..), None) => (step, Position::get_flow()),
        (Some(step), None, None) => (step, Position::get_flow()),
        (None, Some(..), None) => ("start".to_owned(), Position::get_flow()),
        _ => return Err(gen_nom_failure(s, ERROR_GOTO_STEP)),
    };

    Linter::add_goto(
        &Position::get_flow(),
        &Position::get_step(),
        &flow,
        &step,
        interval,
    );

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

    StateContext::clear_state();
    StateContext::inc_rip();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Goto(goto_type, interval)),
            instruction_info,
        ),
    ))
}
