use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_failure, ERROR_GOTO_STEP};
use crate::parser::{
    parse_comments::comment, parse_idents::parse_idents_assignation, get_interval,
    tools::get_string, tools::get_tag, GotoType, StateContext,
};
use crate::linter::data::Linter;
use crate::data::position::Position;

use nom::{branch::alt, error::*, sequence::preceded, *};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_step<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, STEP)(s)?;

    Ok((s, GotoType::Step))
}

fn get_flow<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, FLOW)(s)?;

    Ok((s, GotoType::Flow))
}

fn get_default<'a, E>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E>
where
    E: ParseError<Span<'a>>,
{
    Ok((s, GotoType::Step))
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

    let (s, goto_type) = alt((get_step, get_flow, get_default))(s)?;

    let (s, name) = match parse_idents_assignation(s) {
        Ok((s, name)) => {
            match goto_type {
                GotoType::Flow => {
                    Linter::add_goto(&name.ident, "start", interval);
                }
                GotoType::Step => {
                    Linter::add_goto(&Position::get_flow(), &name.ident, interval);
                }
            }
            (s, name)
        }
        Err(Err::Error((s, _err))) | Err(Err::Failure((s, _err))) => {
            return Err(gen_nom_failure(s, ERROR_GOTO_STEP))
        }
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };

    let instruction_info = InstructionInfo {
        index: StateContext::get_rip(),
        total: 0,
    };

    StateContext::clear_state();
    StateContext::inc_rip();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Goto(goto_type, name)),
            instruction_info,
        ),
    ))
}
