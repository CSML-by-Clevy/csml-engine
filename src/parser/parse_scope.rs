use crate::parser::{
    ast::*, parse_actions::parse_root_functions, parse_ask_response::parse_ask_response,
    parse_comments::comment, parse_for_loop::parse_for, parse_if::parse_if, tokens::*, tools::*,
};
use nom::{
    branch::alt, bytes::complete::tag, error::ParseError, multi::{many0, fold_many0}, sequence::delimited,
    sequence::preceded, *,
};

pub fn parse_root_actions<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<Expr>, E> {
    many0(alt((
        parse_if,
        parse_for,
        // wait_for
        parse_root_functions,
        parse_ask_response,
    )))(s)
}


pub fn parse_root<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Vec<Expr>, FlowType), E> {

    let mut flow_type = FlowType::Recursive;

    let (s, (exprs, boolean)) = fold_many0(
        alt((
            parse_if,
            parse_for,
            // wait_for
            parse_root_functions,
            parse_ask_response,
        )),
        (Vec::new(), false),
        | (mut acc, mut boolean), expr| {
            if let Expr::Block{block_type: BlockType::AskResponse(_), ..} = expr {
                boolean = true;
            }
            acc.push(expr);
            (acc, boolean)
        }
    )(s)?;

    if boolean {
        flow_type = FlowType::Normal;
    }

    Ok((s, (exprs, flow_type)))
}

pub fn parse_implicit_scope<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<Expr>, E> {
    let (s, elem) = parse_root_functions(s)?;
    Ok((s, vec![elem]))
}

pub fn parse_strick_scope<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<Expr>, E> {
    delimited(
        preceded(comment, parse_l_brace),
        parse_root_actions,
        preceded(comment, parse_r_brace),
    )(s)
}

pub fn parse_scope<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Vec<Expr>, E> {
    delimited(
        preceded(comment, tag(L_BRACE)),
        parse_root_actions,
        preceded(comment, parse_r_brace),
    )(s)
}
