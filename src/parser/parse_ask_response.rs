use crate::parser::{
    ast::*,
    parse_actions::parse_root_functions,
    parse_comments::comment,
    parse_ident::parse_ident,
    parse_scope::{parse_scope, parse_strick_scope},
    tokens::*,
    tools::get_interval,
};
use nom::{
    branch::alt, bytes::complete::tag, combinator::complete, combinator::opt, error::ParseError,
    multi::many0, sequence::preceded, *,
};

fn get_option_memory<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Option<Identifier>, E> {
    let (new_span, smart_ident) = parse_ident(s)?;
    if RESERVED.contains(&&*smart_ident.ident) {
        Ok((s, None))
    } else {
        Ok((new_span, Some(smart_ident)))
    }
}

fn parse_ask<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, Option<Identifier>), E> {
    let (s, _) = preceded(comment, tag(ASK))(s)?;
    let (s, opt) = opt(parse_ident)(s)?;
    let (s, start) = get_interval(s)?;
    let (s, block) = parse_scope(s)?;
    let (s, end) = get_interval(s)?;

    Ok((
        s,
        (
            Expr::Block {
                block_type: BlockType::Ask,
                arg: block,
                range: RangeInterval { start, end },
            },
            opt,
        ),
    ))
}

fn parse_response<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, tag(RESPONSE))(s)?;
    let (s, start) = get_interval(s)?;
    let (s, block) = parse_strick_scope(s)?;
    let (s, end) = get_interval(s)?;

    Ok((
        s,
        (Expr::Block {
            block_type: BlockType::Response,
            arg: block,
            range: RangeInterval { start, end },
        }),
    ))
}

fn normal_ask_response<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, start) = get_interval(s)?;
    let (s, ask) = preceded(comment, parse_ask)(s)?;
    let (s, response) = parse_response(s)?;
    let (s, end) = get_interval(s)?;

    Ok((
        s,
        Expr::Block {
            block_type: BlockType::AskResponse(ask.1),
            arg: vec![ask.0, response],
            range: RangeInterval { start, end },
        },
    ))
}

fn short_ask_response<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, tag(ASK))(s)?;
    let (s, ident) = get_option_memory(s)?;
    let (s, start_ask) = get_interval(s)?;
    let (s, ask) = parse_root_functions(s)?;
    let (s, end_ask) = get_interval(s)?;

    let (s, _) = preceded(comment, tag(RESPONSE))(s)?;
    let (s, response) = many0(parse_root_functions)(s)?;
    let (s, end_r) = get_interval(s)?;

    Ok((
        s,
        Expr::Block {
            block_type: BlockType::AskResponse(ident),
            arg: vec![
                Expr::Block {
                    block_type: BlockType::Ask,
                    arg: vec![ask],
                    range: RangeInterval {
                        start: start_ask.clone(),
                        end: end_ask.clone(),
                    },
                },
                Expr::Block {
                    block_type: BlockType::Response,
                    arg: response,
                    range: RangeInterval {
                        start: end_ask,
                        end: end_r.clone(),
                    },
                },
            ],
            range: RangeInterval {
                start: start_ask,
                end: end_r,
            },
        },
    ))
}

pub fn parse_ask_response<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    alt((normal_ask_response, short_ask_response))(s)
}

pub fn wait<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, tag("wait"))(s)?;
    let (s, ident) = complete(parse_ident)(s)?;

    Ok((s, Expr::ObjectExpr(ObjectType::WaitFor(ident))))
}
