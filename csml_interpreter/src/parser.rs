pub mod operator;
pub mod parse_actions;
pub mod parse_braces;
pub mod parse_comments;
pub mod parse_expand_string;
pub mod parse_foreach;
pub mod parse_functions;
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
pub use state_context::{ExecutionState, ExitCondition, StateContext};

use crate::data::position::Position;
use crate::data::{ast::*, tokens::*};
use crate::error_format::*;
use parse_comments::comment;
use parse_scope::parse_root;
use parse_var_types::parse_fn_args;
use tools::*;

use nom::error::ParseError;
use nom::{bytes::complete::tag, multi::fold_many0, sequence::preceded, branch::alt, Err, *};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// TOOL FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_step_name<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, ident) = match parse_idents_assignation(s) {
        Ok((s, ident)) => (s, ident),
        Err(Err::Error((s, _err))) | Err(Err::Failure((s, _err))) => {
            return match s.fragment().is_empty() {
                true => Err(gen_nom_error(s, ERROR_FLOW_STEP)),
                false => Err(gen_nom_failure(s, ERROR_FLOW_STEP)),
            };
        }
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };

    match tag(COLON)(s) {
        Ok((rest, _)) => Ok((rest, ident)),
        Err(Err::Error((s, _err))) | Err(Err::Failure((s, _err))) => {
            Err(gen_nom_failure(s, ERROR_FLOW_STEP))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_flow<'a>(slice: &'a str) -> Result<Flow, ErrorInfo> {
    match start_parsing::<CustomError<Span<'a>>>(Span::new(slice)) {
        Ok((_, (instructions, flow_type))) => Ok(Flow {
            flow_instructions: instructions
                .into_iter()
                .map(|elem| (elem.instruction_type, elem.actions))
                .collect::<HashMap<InstructionType, Expr>>(),

            flow_type,
        }),
        Err(e) => match e {
            Err::Error(err) | Err::Failure(err) => Err(gen_error_info(
                Position::new(Interval::new_as_u32(
                    err.input.location_line(),
                    err.input.get_column() as u32,
                )),
                err.error,
            )),
            Err::Incomplete(_err) => unimplemented!(),
        },
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn parse_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Instruction, E>
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
        Instruction {
            instruction_type: InstructionType::NormalStep(ident.ident),
            actions: Expr::Scope {
                block_type: BlockType::Step,
                scope: actions,
                range: RangeInterval { start, end },
            },
        },
    ))
}

fn parse_function<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Instruction, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag("fn"))(s)?;
    let (s, ident) = preceded(comment, parse_idents_assignation)(s)?;
    let (s, args) = parse_fn_args(s)?;

    // let (s, interval) = get_interval(s)?;

    // Position::set_step(&ident.ident);
    // Linter::add_step(&Position::get_flow(), &ident.ident, interval);
    // StateContext::clear_rip();

    let (s, start) = get_interval(s)?;
    let (s, actions) = preceded(comment, parse_root)(s)?;
    let (s, end) = get_interval(s)?;

    Ok((
        s,
        Instruction {
            instruction_type: InstructionType::FunctionStep{name: ident.ident, args},
            actions: Expr::Scope {
                block_type: BlockType::Step,
                scope: actions,
                range: RangeInterval { start, end },
            },
        },
    ))
}

fn start_parsing<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Vec<Instruction>, FlowType), E> {
    let flow_type = FlowType::Normal;

    let (s, flow) = fold_many0(alt((parse_function, parse_step)), Vec::new(), |mut acc, item| {
        acc.push(item);
        acc
    })(s)?;

    let (last, _) = comment(s)?;
    if !last.fragment().is_empty() {
        Err(gen_nom_failure(last, ERROR_PARSING))
    } else {
        Ok((s, (flow, flow_type)))
    }
}
