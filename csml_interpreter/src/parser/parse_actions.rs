use crate::data::{ast::*, tokens::*};
use crate::error_format::{
    gen_nom_failure, ERROR_ACTION_ARGUMENT, ERROR_REMEMBER, ERROR_RETURN, ERROR_USE,
};
use crate::parser::{
    operator::parse_operator,
    parse_comments::comment,
    parse_foreach::parse_foreach,
    parse_while_loop::parse_while,
    parse_goto::parse_goto,
    parse_previous::parse_previous,
    parse_idents::{parse_idents_assignation, parse_idents_usage},
    parse_if::parse_if,
    parse_path::parse_path,
    parse_var_types::parse_r_bracket,
    tools::{get_interval, get_string, get_tag},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt},
    error::ParseError,
    multi::separated_list,
    sequence::{preceded, terminated, tuple},
    Err, IResult,
};

////////////////////////////////////////////////////////////////////////////////
// TOOL FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn addition_assignment<'a, E>(s: Span<'a>) -> IResult<Span<'a>, AssignType, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(ADDITION_ASSIGNMENT)(s)?;
    Ok((rest, AssignType::AdditionAssignment))
}

fn subtraction_assignment<'a, E>(s: Span<'a>) -> IResult<Span<'a>, AssignType, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(SUBTRACTION_ASSIGNMENT)(s)?;
    Ok((rest, AssignType::SubtractionAssignment))
}

fn assignment<'a, E>(s: Span<'a>) -> IResult<Span<'a>, AssignType, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, ..) = tag(ASSIGN)(s)?;
    Ok((rest, AssignType::Assignment))
}

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

    let (s, assign_type) = preceded(comment, alt((addition_assignment, subtraction_assignment, assignment)))(s)?;
    let (s, expr) = preceded(comment, parse_operator)(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Assign(assign_type, Box::new(ident), Box::new(expr))),
    ))
}

fn parse_remember_as<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Identifier, Box<Expr>), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, operator) = parse_operator(s)?;

    match operator {
        Expr::ObjectExpr(ObjectType::As(idents, expr)) => Ok((s, (idents, expr))),
        _ => Err(gen_nom_failure(s, ERROR_REMEMBER)),
    }
}

fn parse_forget_all<'a, E>(s: Span<'a>) -> IResult<Span<'a>, ForgetMemory, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag("*")(s)?;

    Ok((s, ForgetMemory::ALL))
}

fn parse_forget_single<'a, E>(s: Span<'a>) -> IResult<Span<'a>, ForgetMemory, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, ident) = parse_idents_usage(s)?;

    Ok((s, ForgetMemory::SINGLE(ident)))
}

fn parse_forget_list<'a, E>(s: Span<'a>) -> IResult<Span<'a>, ForgetMemory, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, (vec, _)) = preceded(
        tag(L_BRACKET),
        terminated(
            tuple((
                separated_list(preceded(comment, tag(COMMA)), parse_idents_usage), 
                opt(preceded(comment, tag(COMMA))),
            )),
            preceded(comment, parse_r_bracket),
        ),
    )(s)?;

    Ok((s, ForgetMemory::LIST(vec)))
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
        Err(Err::Failure(e)) => return Err(Err::Failure(e)),
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_do<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, DO)(s)?;

    let (s, expr) = parse_action_argument(s, alt((parse_assignation_with_path, parse_operator)))?;

    let (s, do_type) = match expr {
        Expr::ObjectExpr(ObjectType::As(ident, expr)) => {
            (s, DoType::Update(AssignType::Assignment, Box::new(Expr::IdentExpr(ident)), expr))
        }
        Expr::ObjectExpr(ObjectType::Assign(assign_type, ident, expr)) => (s, DoType::Update(assign_type, ident, expr)),
        _ => (s, DoType::Exec(Box::new(expr))),
    };

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Do(do_type)),
    ))
}

fn parse_remember<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, REMEMBER)(s)?;

    let (s, (idents, expr)) =
        parse_action_argument(s, alt((parse_assignation, parse_remember_as)))?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Remember(idents, expr)),
    ))
}

fn parse_forget<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, interval) = get_interval(s)?;
    let (s, ..) = get_tag(name, FORGET)(s)?;

    let (s, forget_mem) = parse_action_argument(s,
        preceded(
            comment,
            alt((
                parse_forget_all,
                parse_forget_single,
                parse_forget_list
            ))
        )
    )?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Forget(forget_mem, interval)),
    ))
}

fn parse_say<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, SAY)(s)?;

    let (s, expr) = parse_action_argument(s, parse_operator)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Say(Box::new(expr))),
    ))
}

fn parse_debug<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, mut interval) = get_interval(s)?;
    let (s, ..) = get_tag(name, DEBUG_ACTION)(s)?;

    let (s, expr) = parse_action_argument(s, parse_operator)?;
    let (s, end) = get_interval(s)?;
    interval.add_end(end);

    // this vec is temporary until a solution for multiple arguments in debug is found
    let vec = Expr::VecExpr(vec![expr], interval);

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Debug(Box::new(vec), interval)),
    ))
}

//TODO: deprecate use
fn parse_use<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, USE)(s)?;

    let (s, expr) = preceded(comment, parse_operator)(s)?;

    match expr {
        Expr::ObjectExpr(ObjectType::As(..)) => {}
        _ => return Err(gen_nom_failure(s, ERROR_USE)),
    }

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Use(Box::new(expr))),
    ))
}

fn parse_hold<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, inter) = preceded(comment, get_interval)(s)?;
    let (s, name) = get_string(s)?;

    let (s, ..) = get_tag(name, HOLD)(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Hold(inter)),
    ))
}

fn parse_break<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, inter) = preceded(comment, get_interval)(s)?;
    let (s, name) = get_string(s)?;

    let (s, ..) = get_tag(name, BREAK)(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Break(inter)),
    ))
}

fn parse_continue<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, inter) = preceded(comment, get_interval)(s)?;
    let (s, name) = get_string(s)?;

    let (s, ..) = get_tag(name, CONTINUE)(s)?;

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Continue(inter)),
    ))
}

fn parse_return<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, RETURN)(s)?;

    let (s, expr) = match preceded(comment, parse_operator)(s) {
        Ok(value) => value,
        Err(Err::Error(e)) => return Err(Err::Failure(E::add_context(s, ERROR_RETURN, e))),
        Err(Err::Failure(e)) => return Err(Err::Failure(e)),
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };

    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Return(Box::new(expr))),
    ))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_root_functions<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    alt((
        // common actions
        parse_do,
        parse_debug,
        parse_if,
        parse_foreach,
        parse_while,
        // only accessible inside foreach or if scopes
        parse_break,
        parse_continue,
        // only accessible inside normal scopes
        parse_goto,
        parse_previous,
        parse_say,
        parse_remember,
        parse_forget,
        parse_hold,
        // only accessible in functions scopes
        parse_return,
        // soon to be deprecated
        parse_use,
    ))(s)
}