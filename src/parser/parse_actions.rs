use crate::data::{ast::*, tokens::*};
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
    State, StateContext,
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
    let (s, operator) = parse_operator(s)?;
    match operator {
        Expr::ObjectExpr(ObjectType::As(idents, expr)) => Ok((s, (idents, expr))),
        _ => {
            return Err(Err::Failure(E::add_context(
                s,
                "Remember must be assigning to a variable via '=' or 'as': remember key = value || remember value as key",
                E::from_error_kind(s, ErrorKind::Tag),
            )))
        }
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
        index: StateContext::get_index(),
        total: 0,
    };

    StateContext::inc_index();

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
        index: StateContext::get_index(),
        total: 0,
    };

    StateContext::inc_index();

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
        index: StateContext::get_index(),
        total: 0,
    };

    StateContext::inc_index();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Say(Box::new(expr))),
            instruction_info,
        ),
    ))
}

fn parse_use<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, USE)(s)?;

    let (s, expr) = preceded(comment, parse_operator)(s)?;

    match expr {
        Expr::ObjectExpr(ObjectType::As(..)) => {}
        _ => {
            return Err(Err::Failure(E::add_context(
                s,
                "Use must be assigning to a variable via 'as': use value as key",
                E::from_error_kind(s, ErrorKind::Tag),
            )))
        }
    }

    let instruction_info = InstructionInfo {
        index: StateContext::get_index(),
        total: 0,
    };

    StateContext::inc_index();

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
        State::Loop => Err(Err::Failure(E::add_context(
            s,
            "Hold cannot be used inside a foreach",
            E::from_error_kind(s, ErrorKind::Tag),
        ))),
        State::Normal => {
            let instruction_info = InstructionInfo {
                index: StateContext::get_index(),
                total: 0,
            };
            StateContext::inc_index();
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
        State::Loop => {
            let instruction_info = InstructionInfo {
                index: StateContext::get_index(),
                total: 0,
            };
            StateContext::inc_index();
            Ok((
                s,
                (Expr::ObjectExpr(ObjectType::Break(inter)), instruction_info),
            ))
        }
        State::Normal => Err(Err::Failure(E::add_context(
            s,
            "Break can only be used inside a foreach",
            E::from_error_kind(s, ErrorKind::Tag),
        ))),
    }
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
        parse_import,
        parse_hold,
        parse_break,
        parse_if,
        parse_foreach,
    ))(s)
}
