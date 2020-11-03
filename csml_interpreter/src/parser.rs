pub mod operator;
pub mod parse_actions;
pub mod parse_braces;
pub mod parse_built_in;
pub mod parse_comments;
pub mod parse_expand_string;
pub mod parse_foreach;
pub mod parse_goto;
pub mod parse_idents;
pub mod parse_if;
pub mod parse_import;
pub mod parse_literal;
pub mod parse_object;
pub mod parse_parenthesis;
pub mod parse_path;
pub mod parse_scope;
pub mod parse_string;
pub mod parse_var_types;
pub mod state_context;
pub mod tools;

use crate::linter::data::Linter;
use crate::parser::parse_idents::parse_idents_assignation;
pub use state_context::{ExecutionState, ExitCondition, ScopeState, StateContext};

use crate::data::position::Position;
use crate::data::{ast::*, tokens::*};
use crate::error_format::*;
use parse_comments::comment;
use parse_import::parse_import;
use parse_scope::{parse_fn_root, parse_root};
use parse_var_types::parse_fn_args;
use tools::*;

use nom::error::ParseError;
use nom::{branch::alt, bytes::complete::tag, multi::fold_many0, sequence::preceded, Err, *};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// TOOL FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_step_name<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    // this will save the location of teh key word in order to display the error correctly
    let (command_span, _) = comment(s)?;

    let (s2, ident) = match parse_idents_assignation(command_span) {
        Ok((s2, ident)) => (s2, ident),
        Err(Err::Error((s, _err))) | Err(Err::Failure((s, _err))) => {
            return match s.fragment().is_empty() {
                true => Err(gen_nom_error(s, ERROR_PARSING)),
                false => Err(gen_nom_failure(s, ERROR_PARSING)),
            };
        }
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };

    match tag(COLON)(s2) {
        Ok((rest, _)) => Ok((rest, ident)),
        Err(Err::Error((_, _err))) | Err(Err::Failure((_, _err))) => {
            Err(gen_nom_failure(command_span, ERROR_PARSING))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_flow<'a>(slice: &'a str) -> Result<Flow, ErrorInfo> {
    match start_parsing::<CustomError<Span<'a>>>(Span::new(slice)) {
        Ok((_, (instructions, flow_type))) => {
            let flow_instructions =
                instructions
                    .into_iter()
                    .fold(HashMap::new(), |mut flow, elem| {
                        flow.insert(elem.instruction_type, elem.actions);
                        flow
                    });
            Ok(Flow {
                flow_instructions,
                flow_type,
            })
        }
        Err(e) => match e {
            Err::Error(err) | Err::Failure(err) => {
                Err(gen_error_info(
                    Position::new(Interval::new_as_u32(
                        err.input.location_line(),
                        err.input.get_column() as u32,
                    )),
                    convert_error(Span::new(slice), err),
                ))
            }
            Err::Incomplete(_err) => unreachable!(),
        },
    }
}

fn convert_error<'a>(input: Span<'a>, e: CustomError<Span<'a>>) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    let offset = e.input.location_offset();
    let prefix = &input.fragment().as_bytes()[..offset];

    // Count the number of newlines in the first `offset` bytes of input
    let line_number = e.input.location_line();

    // Find the line that includes the subslice:
    // Find the *last* newline before the substring starts
    let line_begin = prefix
        .iter()
        .rev()
        .position(|&b| b == b'\n')
        .map(|pos| offset - pos)
        .unwrap_or(0);

    // Find the full line after that newline
    let line = input.fragment()[line_begin..]
        .lines()
        .next()
        .unwrap_or(&input.fragment()[line_begin..])
        .trim_end();

    // The (1-indexed) column number is the offset of our substring into that line
    let column_number = e.input.get_column();

    write!(
        &mut result,
        "at line {line_number},\n\
            {line}\n\
            {caret:>column$}\n\
            {context}\n\n",
        line_number = line_number,
        context = e.error,
        line = line,
        caret = '^',
        column = column_number,
    )
    // Because `write!` to a `String` is infallible, this `unwrap` is fine.
    .unwrap();

    result
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn parse_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Vec<Instruction>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, ident) = preceded(comment, parse_step_name)(s)?;

    let (s, interval) = get_interval(s)?;

    Position::set_step(&ident.ident);
    Linter::add_step(&Position::get_flow(), &ident.ident, interval);
    StateContext::clear_rip();

    let (s, start) = get_interval(s)?;
    let (s, actions) = preceded(comment, parse_root)(s)?;
    let (s, end) = get_interval(s)?;

    Ok((
        s,
        vec![Instruction {
            instruction_type: InstructionScope::StepScope(ident.ident),
            actions: Expr::Scope {
                block_type: BlockType::Step,
                scope: actions,
                range: RangeInterval { start, end },
            },
        }],
    ))
}

fn parse_function<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<Instruction>, E>
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

    let (s, start) = get_interval(s)?;
    StateContext::set_scope(ScopeState::Function);
    let (s, actions) = preceded(comment, parse_fn_root)(s)?;
    StateContext::set_scope(ScopeState::Normal);
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

fn start_parsing<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Vec<Instruction>, FlowType), E> {
    let flow_type = FlowType::Normal;

    let (s, flow) = fold_many0(
        alt((parse_import, parse_function, parse_step)),
        Vec::new(),
        |mut acc, mut item| {
            acc.append(&mut item);
            acc
        },
    )(s)?;

    let (last, _) = comment(s)?;
    if !last.fragment().is_empty() {
        Err(gen_nom_failure(last, ERROR_PARSING))
    } else {
        Ok((s, (flow, flow_type)))
    }
}
