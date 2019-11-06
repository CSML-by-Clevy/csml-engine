pub mod ast;
pub mod expressions_evaluation;
pub mod literal;
pub mod parse_actions;
pub mod parse_ask_response;
pub mod parse_comments;
pub mod parse_for_loop;
pub mod parse_ident;
pub mod parse_if;
pub mod parse_import;
pub mod parse_literal;
pub mod parse_scope;
pub mod parse_string;
pub mod parse_var_types;
pub mod tokens;
pub mod tools;

use crate::error_format::{CustomError, ErrorInfo};
use ast::*;
use parse_comments::comment;
use parse_ident::parse_ident;
use parse_scope::parse_root;
use tokens::*;
use tools::*;

use nom::error::{ParseError, ErrorKind};
use nom::{bytes::complete::tag, multi::fold_many0, sequence::preceded, Err, *};
use std::collections::HashMap;

fn create_flow_from_instructions<'a>(instructions: Vec<Instruction>, flow_type: FlowType) -> Result<Flow, String> {
    let mut elem = instructions.iter();

    // TODO: see if it can be checked in parsing
    while let Some(val) = elem.next() {
        let elem2 = elem.clone();
        for val2 in elem2 {
            if val.instruction_type == val2.instruction_type {
                return Err("StepDuplicateError".to_owned());
            }
        }
    }

    Ok(Flow {
        flow_instructions: instructions
            .into_iter()
            .map(|elem| (elem.instruction_type, elem.actions))
            .collect::<HashMap<InstructionType, Expr>>(),
        flow_type
    })
}

pub struct Parser;

impl Parser {
    pub fn parse_flow<'a>(slice: &'a str) -> Result<Flow, ErrorInfo> {
        match start_parsing::<CustomError<Span<'a>>>(Span::new(slice)) {
            Ok((s, (instructions, ftype))) => match create_flow_from_instructions(instructions, ftype) {
                Ok(val) => Ok(val),
                Err(error) => Err({
                    ErrorInfo{
                            message: error,
                            interval: Interval{ line: s.line, column: s.get_column() as u32},
                    }
                    
                }),
            },
            Err(e) => match e {
                Err::Error(err) | Err::Failure(err) => {
                    Err(
                        ErrorInfo{
                            message: err.error.to_owned(),
                            interval: Interval{ line: err.input.line, column: err.input.get_column() as u32},
                        }
                    )
                },
                Err::Incomplete(_err) => unimplemented!(),
            },
        }
    }
}


// preceded(comment, )(s)?;
fn parse_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, (Instruction, FlowType), E> {
    let (s, ident) = preceded(comment, parse_ident)(s)?;
    let (s, _) = preceded(comment, tag(COLON))(s)?;
    let (s, start) = get_interval(s)?;
    let (s, (actions, flow_type)) = preceded(comment, parse_root)(s)?;
    let (s, end) = get_interval(s)?;

    Ok((
        s,
        (
            Instruction {
                instruction_type: InstructionType::NormalStep(ident.ident),
                actions: Expr::Block {
                    block_type: BlockType::Step,
                    arg: actions,
                    range: RangeInterval { start, end },
                },
            },
            flow_type
        ),
    ))
}

fn start_parsing<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Vec<Instruction>, FlowType), E> {
    // add comment
    let mut flow_type = FlowType::Recursive;

    let (s, (flow, boolean)) = fold_many0(
        parse_step,
        (Vec::new(), false),
        |(mut acc, mut boolean), (item, ftype)| {
            if let FlowType::Normal = ftype {
                boolean = true;
            };
            acc.push(item);
            (acc, boolean)
        }
    )(s)?;
    if boolean {
        flow_type = FlowType::Normal;
    };
    //check end of file;
    let (last, _) = comment(s)?;
    if last.fragment.len() != 0 {
        // CustomError{input: last, error: "unknown keyword".to_owned()})
        Err(Err::Failure(E::from_error_kind(last, ErrorKind::Tag)))
    } else {
        Ok((s, (flow, flow_type)))
    }

}
