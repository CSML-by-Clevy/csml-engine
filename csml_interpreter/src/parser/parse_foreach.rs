use crate::data::{
    ast::{Expr, Identifier, InstructionInfo, RangeInterval},
    tokens::{Span, COMMA, FOREACH, IN, L_PAREN, R_PAREN},
};
use crate::parser::operator::parse_operator;
use crate::parser::parse_idents::parse_idents_assignation;
use crate::parser::{
    parse_comments::comment, parse_scope::parse_scope, tools::{get_interval, get_tag, get_string}, ExecutionState,
    StateContext,
};
use nom::{bytes::complete::tag, combinator::opt, error::ParseError, sequence::preceded, *};

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
    let (s, start) = get_interval(s)?;

    let (s, _) = preceded(comment, tag(L_PAREN))(s)?;
    let (s, idents) = parse_idents_assignation(s)?;
    let (s, opt) = opt(pars_args)(s)?;
    let (s, _) = preceded(comment, tag(R_PAREN))(s)?;

    let (s, value) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(value, IN)(s)?;

    let (s, expr) = parse_operator(s)?;

    let index = StateContext::get_rip();

    StateContext::inc_rip();

    StateContext::set_state(ExecutionState::Loop);
    let (s, block) = parse_scope(s)?;
    StateContext::set_state(ExecutionState::Normal);

    let (s, end) = get_interval(s)?;

    let new_index = StateContext::get_rip() - 1;
    let instruction_info = InstructionInfo {
        index,
        total: new_index - index,
    };

    Ok((
        s,
        (
            Expr::ForEachExpr(
                idents,
                opt,
                Box::new(expr),
                block,
                RangeInterval { start, end },
            ),
            instruction_info,
        ),
    ))
}
