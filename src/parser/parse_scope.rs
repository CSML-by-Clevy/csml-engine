// parse_ask_response::parse_ask_response,
use crate::parser::{
    ast::*,
    parse_actions::{parse_hook, parse_root_functions},
    parse_comments::comment,
    parse_for_loop::parse_foreach,
    parse_if::parse_if,
    tokens::*,
    tools::*,
};
use nom::{
    branch::alt, bytes::complete::tag, error::ParseError, multi::fold_many0, sequence::delimited,
    sequence::preceded, *,
};

pub fn parse_root_actions<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Block, E> {
    fold_many0(
        alt((parse_if, parse_foreach, parse_root_functions)),
        Block::new(),
        |mut acc: Block, item| {
            acc.commands.push(item);
            acc
        },
    )(s)
}

// TODO: tmp
// pub fn parse_root<'a, E: ParseError<Span<'a>>>(
//     s: Span<'a>,
//     step: String,
// ) -> IResult<Span<'a>, Block, E> {
//     fold_many0(
//         alt((
//             parse_if,
//             parse_foreach,
//             parse_hook,
//             parse_root_functions
//         )),
//         Block::new(), |mut acc: Block, item| {
//             if let Expr::Hook(name) = item {
//                 acc.hooks.push(Hook {
//                     index: acc.commands.len() as i64,
//                     name,
//                     step: step.clone()
//                 })
//             } else {
//                 acc.commands.push(item)
//             }
//             acc
//         }
//     )(s)
// }

pub fn parse_root<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
    step: String,
) -> IResult<Span<'a>, (Block, FlowType), E> {
    let mut flow_type = FlowType::Recursive;

    let (s, (exprs, boolean)) = fold_many0(
        alt((parse_if, parse_foreach, parse_hook, parse_root_functions)),
        (Block::new(), false),
        |(mut acc, mut boolean), item| {
            if let Expr::ObjectExpr(ObjectType::Hold(_)) = item {
                boolean = true;
            }
            if let Expr::Hook(name) = item {
                acc.hooks.push(Hook {
                    index: acc.commands.len() as i64,
                    name,
                    step: step.clone(),
                })
            } else {
                acc.commands.push(item)
            }
            (acc, boolean)
        },
    )(s)?;

    if boolean {
        flow_type = FlowType::Normal;
    }

    Ok((s, (exprs, flow_type)))
}

pub fn parse_implicit_scope<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Block, E> {
    let mut block = Block::new();
    let (s, elem) = parse_root_functions(s)?;
    block.commands.push(elem);
    Ok((s, block))
}

pub fn parse_strick_scope<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Block, E> {
    delimited(
        preceded(comment, parse_l_brace),
        parse_root_actions,
        preceded(comment, parse_r_brace),
    )(s)
}

pub fn parse_scope<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Block, E> {
    delimited(
        preceded(comment, tag(L_BRACE)),
        parse_root_actions,
        preceded(comment, parse_r_brace),
    )(s)
}

pub fn parse_root_scope<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
    name: String,
) -> IResult<Span<'a>, (Block, FlowType), E> {
    delimited2(
        preceded(comment, tag(L_BRACE)),
        parse_root,
        preceded(comment, parse_r_brace),
        name,
    )(s)
}

// TODO: TMP
pub fn delimited2<I, O1, O2, O3, E: ParseError<I>, F, G, H>(
    first: F,
    sep: G,
    second: H,
    name: String,
) -> impl Fn(I) -> IResult<I, O2, E>
where
    F: Fn(I) -> IResult<I, O1, E>,
    G: Fn(I, String) -> IResult<I, O2, E>,
    H: Fn(I) -> IResult<I, O3, E>,
{
    move |input: I| {
        let (input, _) = first(input)?;
        let (input, o2) = sep(input, name.clone())?;
        second(input).map(|(i, _)| (i, o2))
    }
}
