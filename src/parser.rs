pub mod operator;
pub mod parse_actions;
pub mod parse_braces;
pub mod parse_comments;
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

use crate::parser::parse_idents::parse_idents_assignation;
pub use state_context::{ExecutionState, ExitCondition, StateContext};

use crate::data::{ast::*, tokens::*};
use crate::error_format::*;
use crate::linter::Linter;
use parse_comments::comment;
use parse_scope::parse_root;
use tools::*;

use nom::error::ParseError;
use nom::{bytes::complete::tag, multi::fold_many0, sequence::preceded, Err, *};
use std::collections::HashMap;

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
            Err::Error(err) | Err::Failure(err) => Err(ErrorInfo::new(
                Interval {
                    line: err.input.location_line(),
                    column: err.input.get_column() as u32,
                },
                err.error.to_owned(),
            )),
            Err::Incomplete(_err) => unimplemented!(),
        },
    }
}

fn parse_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Instruction, E> {
    let (s, ident) = preceded(comment, parse_idents_assignation)(s)?;
    let (s, _) = preceded(comment, tag(COLON))(s)?;

    let (s, interval) = get_interval(s)?;

    // Linter and Context setup
    Linter::set_step(&Linter::get_flow(), &ident.ident, interval);
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

fn start_parsing<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Vec<Instruction>, FlowType), E> {
    // TODO: handle FlowType::Recursive with Context
    let flow_type = FlowType::Normal;

    let (s, flow) = fold_many0(parse_step, Vec::new(), |mut acc, item| {
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
