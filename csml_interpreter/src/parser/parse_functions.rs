use crate::parser::parse_idents::parse_idents_assignation;

use crate::data::{ast::*, tokens::*};
use crate::error_format::*;
use crate::parser::{
    parse_comments::comment, parse_scope::parse_fn_root, parse_var_types::parse_fn_args, tools::*,
    ScopeState, StateContext,
};

use nom::error::ParseError;
use nom::{bytes::complete::tag, sequence::preceded, Err, *};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_function_prototype<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Identifier, Vec<String>), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag("fn"))(s)?;
    let (s, ident) = preceded(comment, parse_idents_assignation)(s)?;
    let (s, args) = parse_fn_args(s)?;

    let (s, _) = match preceded(comment, tag(COLON))(s) {
        Ok((s, colon)) if *colon.fragment() == COLON => (s, colon),
        Ok(_) => return Err(gen_nom_failure(s, ERROR_FN_COLON)),
        Err(Err::Error((_s, _err))) | Err(Err::Failure((_s, _err))) => {
            return Err(gen_nom_failure(s, ERROR_FN_COLON))
        }
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };

    Ok((s, (ident, args)))
}

pub fn parse_function<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<Instruction>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, start) = preceded(comment, get_interval)(s)?;
    let (s, (ident, args)) = parse_function_prototype(s)?;

    StateContext::set_scope(ScopeState::Function);
    let result = preceded(comment, parse_fn_root)(s);
    StateContext::set_scope(ScopeState::Normal);
    let (s, actions) = result?;

    let (s, end) = get_interval(s)?;

    Ok((
        s,
        vec![Instruction {
            instruction_type: InstructionScope::FunctionScope {
                name: ident.ident,
                args,
            },
            actions: Expr::Scope {
                block_type: BlockType::Function,
                scope: actions,
                range: RangeInterval { start, end },
            },
        }],
    ))
}
