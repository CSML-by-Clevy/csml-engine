use crate::data::warnings::Warnings;
use crate::data::{
    ast::*,
    tokens::*,
    warnings::{WARNING_REMEMBER_AS, WARNING_USE},
};
use crate::error_format::{gen_nom_failure, ERROR_BREAK, ERROR_HOLD, ERROR_REMEMBER, ERROR_USE};
// use crate::linter::Linter;
use crate::parser::{
    operator::parse_operator,
    parse_comments::comment,
    parse_foreach::parse_foreach,
    parse_goto::parse_goto,
    parse_idents::parse_idents_assignation,
    parse_if::parse_if,
    parse_import::parse_import,
    parse_path::parse_path,
    tools::get_interval,
    tools::{get_string, get_tag},
    ExecutionState, StateContext,
};
use nom::{branch::alt, bytes::complete::tag, error::*, sequence::preceded, *};

////////////////////////////////////////////////////////////////////////////////
// TOOL FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_assignation<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Identifier, Box<Expr>), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = parse_idents_assignation(s)?;
    let (s, _) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, expr) = preceded(comment, parse_operator)(s)?;

    Ok((s, (name, Box::new(expr))))
}

fn parse_assignation_with_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = parse_idents_assignation(s)?;
    let (s, ident) = parse_path(s, Expr::IdentExpr(name))?;

    let (s, _) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, expr) = preceded(comment, parse_operator)(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Assign(Box::new(ident), Box::new(expr))),
    ))
}

fn parse_remember_as<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Identifier, Box<Expr>), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;

    Warnings::add(WARNING_REMEMBER_AS, interval);

    let (s, operator) = parse_operator(s)?;

    match operator {
        Expr::ObjectExpr(ObjectType::As(idents, expr)) => Ok((s, (idents, expr))),
        _ => Err(gen_nom_failure(s, ERROR_REMEMBER)),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_do<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, DO)(s)?;

    let (s, expr) = preceded(comment, alt((parse_assignation_with_path, parse_operator)))(s)?;

    let (s, do_type) = match expr {
        Expr::ObjectExpr(ObjectType::As(ident, expr)) => {
            (s, DoType::Update(Box::new(Expr::IdentExpr(ident)), expr))
        }
        Expr::ObjectExpr(ObjectType::Assign(ident, expr)) => (s, DoType::Update(ident, expr)),
        _ => (s, DoType::Exec(Box::new(expr))),
    };

    let instruction_info = InstructionInfo {
        index: StateContext::get_rip(),
        total: 0,
    };

    StateContext::inc_rip();

    Ok((
        s,
        (Expr::ObjectExpr(ObjectType::Do(do_type)), instruction_info),
    ))
}

fn parse_remember<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, REMEMBER)(s)?;

    let (s, (idents, expr)) = preceded(comment, alt((parse_assignation, parse_remember_as)))(s)?;

    let instruction_info = InstructionInfo {
        index: StateContext::get_rip(),
        total: 0,
    };

    StateContext::inc_rip();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Remember(idents, expr)),
            instruction_info,
        ),
    ))
}

fn parse_say<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, SAY)(s)?;

    let (s, expr) = preceded(comment, parse_operator)(s)?;

    let instruction_info = InstructionInfo {
        index: StateContext::get_rip(),
        total: 0,
    };

    StateContext::inc_rip();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Say(Box::new(expr))),
            instruction_info,
        ),
    ))
}

//TODO: deprecat use
fn parse_use<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, USE)(s)?;
    let (s, interval) = get_interval(s)?;

    Warnings::add(WARNING_USE, interval);

    let (s, expr) = preceded(comment, parse_operator)(s)?;

    match expr {
        Expr::ObjectExpr(ObjectType::As(..)) => {}
        _ => return Err(gen_nom_failure(s, ERROR_USE)),
    }

    let instruction_info = InstructionInfo {
        index: StateContext::get_rip(),
        total: 0,
    };

    StateContext::inc_rip();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Use(Box::new(expr))),
            instruction_info,
        ),
    ))
}

fn parse_hold<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, inter) = get_interval(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;

    let (s, ..) = get_tag(name, HOLD)(s)?;

    match StateContext::get_state() {
        ExecutionState::Loop => Err(gen_nom_failure(s, ERROR_HOLD)),
        ExecutionState::Normal => {
            let instruction_info = InstructionInfo {
                index: StateContext::get_rip(),
                total: 0,
            };
            StateContext::inc_rip();
            Ok((
                s,
                (Expr::ObjectExpr(ObjectType::Hold(inter)), instruction_info),
            ))
        }
    }
}

fn parse_break<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, inter) = get_interval(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;

    let (s, ..) = get_tag(name, BREAK)(s)?;

    match StateContext::get_state() {
        ExecutionState::Loop => {
            let instruction_info = InstructionInfo {
                index: StateContext::get_rip(),
                total: 0,
            };
            StateContext::inc_rip();
            Ok((
                s,
                (Expr::ObjectExpr(ObjectType::Break(inter)), instruction_info),
            ))
        }
        ExecutionState::Normal => Err(gen_nom_failure(s, ERROR_BREAK)),
    }
}

fn parse_return<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, RETURN)(s)?;

    let (s, expr) = preceded(comment, parse_operator)(s)?;

    let instruction_info = InstructionInfo {
        index: StateContext::get_rip(),
        total: 0,
    };

    StateContext::inc_rip();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Return(Box::new(expr))),
            instruction_info,
        ),
    ))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_root_functions<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    //TODO: catch use of 'return' and return error informing user that this functions are not allowed in the normal scope
    alt((
        parse_do,
        parse_goto,
        parse_remember,
        parse_say,
        parse_use,
        parse_import,
        parse_hold,
        parse_break,
        parse_if,
        parse_foreach,
        // parse_return,
    ))(s)
}

pub fn parse_fn_root_functions<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    //TODO: catch use of goto, remember, use, .. and return error informing user that this functions are not allowed in the Fn scope
    alt((
        parse_do,
        // parse_goto,
        // parse_remember,
        parse_say,
        // parse_use,
        // parse_import,
        // parse_hold,
        // parse_break,
        parse_if,
        parse_foreach,
        parse_return,
    ))(s)
}
