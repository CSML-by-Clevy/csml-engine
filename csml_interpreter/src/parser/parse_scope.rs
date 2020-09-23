use crate::data::{ast::*, tokens::*};
use crate::parser::parse_braces::parse_r_brace;
use crate::parser::{
    parse_actions::{parse_fn_root_functions, parse_root_functions},
    parse_comments::comment,
};
use nom::{
    bytes::complete::tag, error::ParseError, multi::fold_many0, sequence::delimited,
    sequence::preceded, *,
};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_root<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>>,
{
    fold_many0(
        parse_root_functions,
        Block::default(),
        |mut acc: Block, (item, instruction_info)| {
            acc.commands.push((item, instruction_info));
            acc
        },
    )(s)
}

pub fn parse_fn_root<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>>,
{
    fold_many0(
        parse_fn_root_functions,
        Block::default(),
        |mut acc: Block, (item, instruction_info)| {
            acc.commands.push((item, instruction_info));
            acc
        },
    )(s)
}

pub fn parse_implicit_scope<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>>,
{
    let mut acc = Block::default();
    let (s, (item, instruction_info)) = parse_root_functions(s)?;
    acc.commands.push((item, instruction_info));
    Ok((s, acc))
}

pub fn parse_scope<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>>,
{
    delimited(
        preceded(comment, tag(L_BRACE)),
        parse_root,
        preceded(comment, parse_r_brace),
    )(s)
}
