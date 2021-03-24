use crate::data::{ast::*, primitive::closure::PrimitiveClosure, tokens::*};
use crate::parser::{
    tools::*,
    parse_comments::comment,
    parse_braces::{parse_r_brace},
    parse_scope::parse_fn_root, ScopeState, StateContext,
};
use nom::{
    bytes::complete::tag,
    combinator::{opt},
    error::ParseError,
    multi::separated_list,
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn parse_closure_args<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<String>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, (vec, _)) = preceded(
        tag(L_PAREN),
        terminated(
            tuple((
                separated_list(preceded(comment, tag(COMMA)), preceded(comment, get_string)),
                opt(preceded(comment, tag(COMMA))),
            )),
            preceded(comment, tag(R_PAREN)),
        ),
    )(s)?;

    Ok((s, vec))
}

pub fn parse_closure<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, mut interval) = preceded(comment, get_interval)(s)?;
    let (s, args) = parse_closure_args(s)?;

    let (s, _) = preceded(comment, tag(L_BRACE))(s)?;

    StateContext::set_scope(ScopeState::Function);
    let result = preceded(comment, parse_fn_root)(s);
    StateContext::set_scope(ScopeState::Normal);
    let (s, func) = result?;

    let (s, _) = preceded(comment, parse_r_brace)(s)?;

    let (s, end) = get_interval(s)?;
    interval.add_end(end);

    let closure = Expr::LitExpr(
        PrimitiveClosure::get_literal(
            args,
            Box::new(Expr::Scope {
                block_type: BlockType::Function,
                scope: func,
                range: interval,
            }),
            interval,
            None
        )
    );

    Ok((
        s,
        closure
    ))
}