use crate::parser::{
    ast::*,
    parse_comments::comment,
    parse_ident::{get_tag, parse_ident, parse_ident_no_check, get_string},
    parse_import::parse_import,
    parse_var_types::{parse_basic_expr, parse_expr_list, parse_var_expr},
    // expressions_evaluation::operator_precedence,
    tokens::*,
    tools::get_interval,
    GotoType,
    singleton::*,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{complete, opt},
    error::*,
    sequence::preceded,
    *,
};

pub fn parse_assignation<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = parse_ident_no_check(s)?;
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

fn parse_goto<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, GOTO)(s)?;
    let (s, goto_type) = alt((get_step, get_flow, get_hook, get_default))(s)?;
    let (s, name) = match parse_ident(s) {
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

    State::clear();

    Ok((s, Expr::ObjectExpr(ObjectType::Goto(goto_type, name))))
}

fn parse_say<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, SAY)(s)?;
    let (s, expr) = parse_var_expr(s)?;
    Ok((s, Expr::ObjectExpr(ObjectType::Say(Box::new(expr)))))
}

fn parse_use<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, USE)(s)?;
    let (s, expr) = parse_var_expr(s)?;
    Ok((s, Expr::ObjectExpr(ObjectType::Use(Box::new(expr)))))
}

fn parse_do_update<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ..) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, new) = parse_var_expr(s)?;
    Ok((s, new))
}

fn parse_do<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, DO)(s)?;
    let (s, old) = parse_basic_expr(s)?;

    let (s, do_type) = match opt(parse_do_update)(s)? {
        (s, Some(new)) => (s, DoType::Update(Box::new(old), Box::new(new))),
        (s, None) => (s, DoType::Exec(Box::new(old))),
    };

    Ok((s, Expr::ObjectExpr(ObjectType::Do(do_type))))
}

fn parse_hold<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, inter) = get_interval(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, HOLD)(s)?;

    match State::get() {
        State::Loop => Err(Err::Failure(E::add_context(s, "Hold cannot be used inside a foreach", E::from_error_kind(s, ErrorKind::Tag)))),
        State::Normal => Ok((s, Expr::ObjectExpr(ObjectType::Hold(inter)))),
    }
}

fn parse_break<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, inter) = get_interval(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, BREAK)(s)?;

    match State::get() {
        State::Loop => Ok((s, Expr::ObjectExpr(ObjectType::Break(inter)))),
        State::Normal => Err(Err::Failure(E::add_context(s, "Break can only be used inside a foreach", E::from_error_kind(s, ErrorKind::Tag)))),
    }
}

fn parse_as_remember<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, (Expr, Identifier), E> {
    let (s, expr) = parse_var_expr(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, _) = get_tag(name, AS)(s)?;
    let (s, ident) = preceded(comment, complete(parse_ident))(s)?;
    Ok((s, (expr, ident)))
}

fn parse_assign_remember<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, (Expr, Identifier), E> {
    let (s, ident) = preceded(comment, complete(parse_ident))(s)?;
    let (s, _) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, expr) = parse_var_expr(s)?;
    Ok((s, (expr, ident)))
}

fn parse_remember<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, REMEMBER)(s)?;
    let (s, (expr, ident)) = alt((parse_as_remember, parse_assign_remember))(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Remember(ident, Box::new(expr))),
    ))
}

pub fn parse_actions<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = parse_ident_no_check(s)?;
    let (s, expr) = parse_expr_list(s)?;
    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Normal(name, Box::new(expr))),
    ))
}

pub fn parse_hook<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ..) = preceded(comment, tag("@"))(s)?;
    //TODO: add error if ident not found
    let (s, name) = get_string(s)?;

    Ok((s, Expr::Hook(name)))
}

pub fn parse_root_functions<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Expr, E> {
    alt((
        parse_say,
        parse_remember,
        parse_import,
        parse_goto,
        parse_use,
        parse_do,
        parse_hold,
        parse_break,
    ))(s)
}
