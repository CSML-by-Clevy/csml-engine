use crate::parser::{
    ast::*,
    context::*,
    expressions_evaluation::operator_precedence,
    parse_comments::comment,
    parse_for_loop::parse_foreach,
    parse_idents::{get_string, get_tag, parse_idents, parse_idents_no_check},
    parse_if::parse_if,
    parse_import::parse_import,
    parse_var_types::{parse_basic_expr, parse_expr_list, parse_var_expr},
    tokens::*,
    tools::get_interval,
    GotoType,
};
use nom::{branch::alt, bytes::complete::tag, combinator::opt, error::*, sequence::preceded, *};

pub fn parse_assignation<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = parse_idents_no_check(s)?;
    let (s, _) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, expr) = parse_var_expr(s)?;
    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Assign(name, Box::new(expr))),
    ))
}

fn get_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, STEP)(s)?;
    Ok((s, GotoType::Step))
}

fn get_hook<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E> {
    let (s, ..) = preceded(comment, tag("@"))(s)?;
    Ok((s, GotoType::Hook))
}

fn get_flow<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, FLOW)(s)?;
    Ok((s, GotoType::Flow))
}

fn get_default<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E> {
    Ok((s, GotoType::Step))
}

fn parse_goto<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, GOTO)(s)?;
    let (s, goto_type) = alt((get_step, get_flow, get_hook, get_default))(s)?;
    let (s, name) = match parse_idents(s) {
        Ok(vars) => vars,
        Err(Err::Error(err)) | Err(Err::Failure(err)) => {
            return Err(Err::Error(E::add_context(
                s,
                "missing step name after goto",
                err,
            )))
        }
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };

    let instruction_info = InstructionInfo {
        index: Context::get_index(),
        total: 0,
    };

    Context::clear_state();
    Context::inc_index();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Goto(goto_type, name)),
            instruction_info,
        ),
    ))
}

fn parse_say<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, SAY)(s)?;

    let (s, expr) = parse_var_expr(s)?;

    let instruction_info = InstructionInfo {
        index: Context::get_index(),
        total: 0,
    };

    Context::inc_index();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Say(Box::new(expr))),
            instruction_info,
        ),
    ))
}

fn parse_use<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, USE)(s)?;
    let (s, expr) = parse_var_expr(s)?;

    let instruction_info = InstructionInfo {
        index: Context::get_index(),
        total: 0,
    };

    Context::inc_index();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Use(Box::new(expr))),
            instruction_info,
        ),
    ))
}

fn parse_do_update<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ..) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, new) = operator_precedence(s)?;
    Ok((s, new))
}

fn parse_do<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, DO)(s)?;
    let (s, old) = parse_basic_expr(s)?;

    let (s, do_type) = match opt(parse_do_update)(s)? {
        (s, Some(Expr::ObjectExpr(ObjectType::As(ident, expr))))
        | (s, Some(Expr::ObjectExpr(ObjectType::Assign(ident, expr)))) => {
            (s, DoType::Update(Box::new(Expr::IdentExpr(ident)), expr))
        }
        (s, Some(new)) => (s, DoType::Update(Box::new(old), Box::new(new))),
        (s, None) => (s, DoType::Exec(Box::new(old))),
    };

    let instruction_info = InstructionInfo {
        index: Context::get_index(),
        total: 0,
    };

    Context::inc_index();

    Ok((
        s,
        (Expr::ObjectExpr(ObjectType::Do(do_type)), instruction_info),
    ))
}

fn parse_hold<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, inter) = get_interval(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, HOLD)(s)?;

    match Context::get_state() {
        State::Loop => Err(Err::Failure(E::add_context(
            s,
            "Hold cannot be used inside a foreach",
            E::from_error_kind(s, ErrorKind::Tag),
        ))),
        State::Normal => {
            let instruction_info = InstructionInfo {
                index: Context::get_index(),
                total: 0,
            };
            Context::inc_index();
            Ok((
                s,
                (Expr::ObjectExpr(ObjectType::Hold(inter)), instruction_info),
            ))
        }
    }
}

fn parse_break<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, inter) = get_interval(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, BREAK)(s)?;

    match Context::get_state() {
        State::Loop => {
            let instruction_info = InstructionInfo {
                index: Context::get_index(),
                total: 0,
            };
            Context::inc_index();
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

fn parse_remember<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, REMEMBER)(s)?;
    let (s, expr) = parse_var_expr(s)?;
    let (expr, idents) = match expr {
        Expr::ObjectExpr(ObjectType::Assign(idents, expr))
        | Expr::ObjectExpr(ObjectType::As(idents, expr)) => (expr, idents),
        _ => {
            return Err(Err::Failure(E::add_context(
                s,
                "Remember bad format",
                E::from_error_kind(s, ErrorKind::Tag),
            )))
        }
    };

    let instruction_info = InstructionInfo {
        index: Context::get_index(),
        total: 0,
    };

    Context::inc_index();

    Ok((
        s,
        (
            Expr::ObjectExpr(ObjectType::Remember(idents, expr)),
            instruction_info,
        ),
    ))
}

pub fn parse_functions<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, interval) = get_interval(s)?;
    let (s, name) = get_string(s)?;
    let (s, expr) = parse_expr_list(s)?;
    let func = Function {
        name,
        interval,
        args: Box::new(expr),
    };

    Ok((s, Expr::ObjectExpr(ObjectType::Normal(func))))
}

pub fn parse_root_functions<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    alt((
        parse_if,
        parse_foreach,
        parse_import,
        parse_say,
        parse_remember,
        parse_goto,
        parse_use,
        parse_do,
        parse_hold,
        parse_break,
    ))(s)
}
