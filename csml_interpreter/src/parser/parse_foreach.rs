use crate::data::{
    ast::{Expr, Identifier, InstructionInfo},
    tokens::{Span, COMMA, FOREACH, IN, L_PAREN, R_PAREN},
};
use crate::parser::operator::parse_operator;
use crate::parser::parse_idents::parse_idents_assignation;
use crate::parser::{
    parse_comments::comment,
    parse_scope::parse_scope,
    tools::{get_interval, get_string, get_tag},
    StateContext,
};
use nom::{
    bytes::complete::tag,
    combinator::{cut, opt},
    error::ParseError,
    sequence::preceded,
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn pars_args<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag(COMMA))(s)?;
    let (s, idents) = parse_idents_assignation(s)?;

    Ok((s, idents))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_foreach<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag(FOREACH))(s)?;
    let (s, mut interval) = get_interval(s)?;

    let (s, _) = cut(preceded(comment, tag(L_PAREN)))(s)?;
    let (s, idents) = cut(parse_idents_assignation)(s)?;
    let (s, opt) = opt(pars_args)(s)?;
    let (s, _) = cut(preceded(comment, tag(R_PAREN)))(s)?;

    let (s, value) = cut(preceded(comment, get_string))(s)?;
    let (s, ..) = cut(get_tag(value, IN))(s)?;

    let (s, expr) = cut(parse_operator)(s)?;

    let index = StateContext::get_rip();

    StateContext::inc_rip();

    // StateContext::set_state(ExecutionState::Loop);
    // let scope_type = StateContext::get_scope();
    let (s, block) = parse_scope(s)?;
    // match scope_type {
    //     ScopeState::Normal => parse_scope(s)?,
    //     ScopeState::Function => parse_fn_scope(s)?,
    // };
    // StateContext::set_state(ExecutionState::Normal);

    let (s, end) = get_interval(s)?;
    interval.add_end(end);

    let new_index = StateContext::get_rip() - 1;
    let instruction_info = InstructionInfo {
        index,
        total: new_index - index,
    };

    Ok((
        s,
        (
            Expr::ForEachExpr(idents, opt, Box::new(expr), block, interval),
            instruction_info,
        ),
    ))
}
