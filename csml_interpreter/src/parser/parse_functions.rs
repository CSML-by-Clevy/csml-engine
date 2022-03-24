use crate::parser::parse_idents::parse_idents_assignation;

use crate::data::{ast::*, tokens::*};
use crate::error_format::*;
use crate::parser::{
    parse_braces::parse_r_brace,
    parse_comments::comment, parse_scope::parse_root,
    parse_var_types::parse_fn_args, tools::*,
};

use nom::error::{ParseError, ContextError};
use nom::{
    bytes::complete::tag, sequence::{preceded, delimited},
    branch::alt, Err, IResult
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_function_scope_colon<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, _) = match preceded(comment, tag(COLON))(s) {
        Ok((s, colon)) if *colon.fragment() == COLON => (s, colon),
        Ok(_) => return Err(gen_nom_error(s, ERROR_FN_COLON)),

        Err(Err::Error((_s, _err))) | Err(Err::Failure((_s, _err))) => {
            return Err(gen_nom_error(s, ERROR_FN_COLON))
        }
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };

    preceded(comment, parse_root)(s)
}

fn parse_function_scope<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Block, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    delimited(
        preceded(comment, tag(L_BRACE)),
        parse_root,
        preceded(comment, parse_r_brace),
    )(s)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_function<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<Instruction>, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, mut interval) = preceded(comment, get_interval)(s)?;

    let (s, _) = preceded(comment, tag("fn"))(s)?;
    let (s, ident) = preceded(comment, parse_idents_assignation)(s)?;
    let (s, args) = parse_fn_args(s)?;

    let (s, scope) = alt((parse_function_scope_colon, parse_function_scope))(s)?;

    let (s, end) = get_interval(s)?;

    interval.add_end(end);

    Ok((
        s,
        vec![Instruction {
            instruction_type: InstructionScope::FunctionScope {
                name: ident.ident,
                args,
            },
            actions: Expr::Scope {
                block_type: BlockType::Function,
                scope,
                range: interval,
            },
        }],
    ))
}
