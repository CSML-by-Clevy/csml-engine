use crate::data::warnings::Warnings;
use crate::data::{
    ast::*,
    tokens::*,
    warnings::{WARNING_REMEMBER_AS, WARNING_USE},
};
use crate::error_format::{
    gen_nom_failure, ERROR_ACTION_ARGUMENT, ERROR_BREAK, ERROR_FN_SCOPE, ERROR_HOLD,
    ERROR_REMEMBER, ERROR_RETURN, ERROR_SCOPE, ERROR_USE,
};
// use crate::linter::Linter;
use crate::parser::{
    operator::parse_operator,
    parse_comments::comment,
    parse_foreach::parse_foreach,
    parse_goto::parse_goto,
    parse_idents::parse_idents_assignation,
    parse_if::parse_if,
    parse_path::parse_path,
    tools::{get_interval, get_string, get_tag},
    ExecutionState, StateContext,
};
use nom::{branch::alt, bytes::complete::tag, error::*, sequence::preceded, Err, IResult};

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

fn parse_action_argument<'a, E, F, G>(s: Span<'a>, func: F) -> IResult<Span<'a>, G, E>
where
    E: ParseError<Span<'a>>,
    F: Fn(Span<'a>) -> IResult<Span<'a>, G, E>,
{
    match preceded(comment, func)(s) {
        Ok(value) => Ok(value),
        Err(Err::Error(e)) => {
            return Err(Err::Failure(E::add_context(s, ERROR_ACTION_ARGUMENT, e)))
        }
        Err(Err::Failure(e)) => return Err(Err::Failure(E::append(s, ErrorKind::Tag, e))),
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
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

    let (s, expr) = parse_action_argument(s, alt((parse_assignation_with_path, parse_operator)))?;

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

    let (s, (idents, expr)) =
        parse_action_argument(s, alt((parse_assignation, parse_remember_as)))?;

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

    let (s, expr) = parse_action_argument(s, parse_operator)?;

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

//TODO: deprecate use
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
    let (s, inter) = preceded(comment, get_interval)(s)?;
    let (s, name) = get_string(s)?;

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
    let (s, inter) = preceded(comment, get_interval)(s)?;
    let (s, name) = get_string(s)?;

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

fn parse_continue<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, inter) = preceded(comment, get_interval)(s)?;
    let (s, name) = get_string(s)?;

    let (s, ..) = get_tag(name, CONTINUE)(s)?;

    match StateContext::get_state() {
        ExecutionState::Loop => {
            let instruction_info = InstructionInfo {
                index: StateContext::get_rip(),
                total: 0,
            };
            StateContext::inc_rip();
            Ok((
                s,
                (Expr::ObjectExpr(ObjectType::Continue(inter)), instruction_info),
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

    let (s, expr) = match preceded(comment, parse_operator)(s) {
        Ok(value) => value,
        Err(Err::Error(e)) => return Err(Err::Failure(E::add_context(s, ERROR_RETURN, e))),
        Err(Err::Failure(e)) => return Err(Err::Failure(E::append(s, ErrorKind::Tag, e))),
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };

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

fn catch_scope_fn_common_mistakes<'a, E>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;

    if FN_SCOPE_REJECTED.contains(&name.as_ref()) {
        return Err(gen_nom_failure(s, ERROR_FN_SCOPE));
    }

    Err(Err::Error(E::from_error_kind(s, ErrorKind::Tag)))
}

fn catch_scope_common_mistakes<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s2, ..) = comment(s)?;
    let (_, name) = get_string(s)?;

    if SCOPE_REJECTED.contains(&name.as_ref()) {
        return Err(gen_nom_failure(s, ERROR_SCOPE));
    }

    Err(Err::Error(E::from_error_kind(s2, ErrorKind::Tag)))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_root_functions<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    alt((
        parse_do,
        parse_goto,
        parse_remember,
        parse_say,
        parse_use,
        parse_hold,
        parse_break,
        parse_continue,
        parse_if,
        parse_foreach,
        catch_scope_common_mistakes,
    ))(s)
}

pub fn parse_fn_root_functions<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    alt((
        parse_do,
        parse_if,
        parse_foreach,
        parse_break,
        parse_continue,
        parse_return,
        catch_scope_fn_common_mistakes,
    ))(s)
}
