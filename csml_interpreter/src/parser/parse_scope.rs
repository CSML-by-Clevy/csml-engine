use crate::data::{ast::*, tokens::*};
use crate::parser::parse_braces::parse_r_brace;
use crate::parser::{
    parse_actions::parse_root_functions,
    parse_comments::comment,
    state_context::count_commands,
    tools::{get_interval, parse_error},
};
use nom::{
    bytes::complete::tag,
    error::{ContextError, ParseError},
    multi::fold_many0,
    sequence::delimited,
    sequence::preceded,
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_root<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let result = fold_many0(
        parse_root_functions,
        Block::default,
        |mut acc: Block, mut command| {
            let mut index = acc.commands_count;

            let mut instruction_info = InstructionInfo { index, total: 0 };

            count_commands(&mut command, &mut index, &mut instruction_info);

            acc.commands.push((command, instruction_info));
            acc.commands_count = index;
            acc
        },
    )(s);

    result
}

pub fn parse_implicit_scope<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let mut acc = Block::default();
    let (s, item) = parse_root_functions(s)?;

    let instruction_info = InstructionInfo { index: 0, total: 0 };

    acc.commands.push((item, instruction_info));
    Ok((s, acc))
}

pub fn parse_scope<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (start, _) = get_interval(s)?;

    parse_error(
        start,
        s,
        delimited(
            preceded(comment, tag(L_BRACE)),
            parse_root,
            preceded(comment, parse_r_brace),
        ),
    )
}
