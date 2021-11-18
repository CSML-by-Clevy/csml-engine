pub mod operator;
pub mod parse_actions;
pub mod parse_braces;
pub mod parse_built_in;
pub mod parse_closure;
pub mod parse_comments;
pub mod parse_foreach;
pub mod parse_while_loop;
pub mod parse_functions;
pub mod parse_goto;
pub mod parse_idents;
pub mod parse_if;
pub mod parse_import;
pub mod parse_literal;
pub mod parse_object;
pub mod parse_parenthesis;
pub mod parse_path;
pub mod parse_previous;
pub mod parse_scope;
pub mod parse_string;
pub mod parse_var_types;
pub mod state_context;
pub mod step_checksum;
pub mod tools;

use crate::parser::parse_idents::parse_idents_assignation;
pub use state_context::ExitCondition;

use crate::data::position::Position;
use crate::data::{ast::*, tokens::*};
use crate::error_format::*;
use crate::interpreter::variable_handler::interval::interval_from_expr;
use parse_comments::comment;
use parse_functions::parse_function;
use parse_import::parse_import;
use parse_scope::parse_root;
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
    // this will save the location of the keyword in order to display the error correctly
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

pub fn parse_flow<'a>(slice: &'a str, flow_name: &'a str) -> Result<Flow, ErrorInfo> {
    match start_parsing::<CustomError<Span<'a>>>(Span::new(slice)) {
        Ok((_, (instructions, flow_type))) => {
            let flow_instructions =
                instructions
                    .into_iter()
                    .fold(HashMap::new(), |mut flow, elem| {
                        let instruction_interval = interval_from_expr(&elem.actions);
                        let instruction_info = elem.instruction_type.get_info();

                        if let Some(old_instruction) =
                            flow.insert(elem.instruction_type, elem.actions)
                        {
                            // this is done in order to store all duplicated instruction during parsing
                            // and use by the linter to display them all as errors
                            flow.insert(
                                InstructionScope::DuplicateInstruction(
                                    instruction_interval,
                                    instruction_info,
                                ),
                                old_instruction,
                            );
                        };
                        flow
                    });

            Ok(Flow {
                flow_instructions,
                flow_type,
            })
        }
        Err(e) => match e {
            Err::Error(err) | Err::Failure(err) => {
                let (end_line, end_column) = match err.end {
                    Some(end) => (Some(end.location_line()), Some(end.get_column() as u32)),
                    None => (None, None),
                };

                Err(gen_error_info(
                    Position::new(
                        Interval::new_as_u32(
                            err.input.location_line(),
                            err.input.get_column() as u32,
                            err.input.location_offset(),
                            end_line,
                            end_column,
                        ),
                        flow_name,
                    ),
                    convert_error_from_span(Span::new(slice), err),
                ))
            }
            Err::Incomplete(_err) => unreachable!(),
        },
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn parse_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Vec<Instruction>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, mut interval) = preceded(comment, get_interval)(s)?;
    let (s, ident) = preceded(comment, parse_step_name)(s)?;

    let (s, actions) = preceded(comment, parse_root)(s)?;
    let (s, end) = get_interval(s)?;
    interval.add_end(end);

    Ok((
        s,
        vec![Instruction {
            instruction_type: InstructionScope::StepScope(ident.ident),
            actions: Expr::Scope {
                block_type: BlockType::Step,
                scope: actions,
                range: interval,
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
