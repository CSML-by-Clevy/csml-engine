use crate::parser::{
    ast::*, parse_actions::parse_root_functions, parse_comments::comment, tokens::*, tools::*,
};
use nom::{
    bytes::complete::tag, error::ParseError, multi::fold_many0, sequence::delimited,
    sequence::preceded, *,
};

pub fn parse_root<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Block, E> {
    fold_many0(
        parse_root_functions,
        Block::default(),
        |mut acc: Block, (item, instruction_info)| {
            acc.commands.push((item, instruction_info));
            acc
        },
    )(s)
}

pub fn parse_implicit_scope<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Block, E> {
    let mut acc = Block::default();
    let (s, (item, instruction_info)) = parse_root_functions(s)?;
    acc.commands.push((item, instruction_info));
    Ok((s, acc))
}

pub fn parse_strict_scope<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Block, E> {
    delimited(
        preceded(comment, parse_l_brace),
        parse_root,
        preceded(comment, parse_r_brace),
    )(s)
}

pub fn parse_scope<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Block, E> {
    delimited(
        preceded(comment, tag(L_BRACE)),
        parse_root,
        preceded(comment, parse_r_brace),
    )(s)
}

pub fn parse_root_scope<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
    _name: String,
) -> IResult<Span<'a>, Block, E> {
    delimited(
        preceded(comment, tag(L_BRACE)),
        parse_root,
        preceded(comment, parse_r_brace),
    )(s)
}
