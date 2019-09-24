pub mod ast;
pub mod literal;
pub mod expressions_evaluation;
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

use crate::comment;
use crate::error_format::{data::*, *};
use ast::*;
use parse_ident::parse_ident;
use parse_scope::parse_root_actions;
use tokens::*;
use tools::*;

use nom::types::*;
use nom::{Err, *};
use std::collections::HashMap;

fn create_flow_from_instructions(instructions: Vec<Instruction>) -> Result<Flow, ErrorInfo> {
    let mut elem = instructions.iter();
    while let Some(val) = elem.next() {
        let elem2 = elem.clone();
        for val2 in elem2 {
            if val.instruction_type == val2.instruction_type {
                return Err(format_error(
                    Interval { line: 0, column: 0 },
                    ErrorKind::Custom(ParserErrorType::StepDuplicateError as u32),
                    &vec![],
                ));
            }
        }
    }

    Ok(Flow {
        flow_instructions: instructions
            .into_iter()
            .map(|elem| (elem.instruction_type, elem.actions))
            .collect::<HashMap<InstructionType, Expr>>(),
    })
}
pub struct Parser;

impl Parser {
    pub fn parse_flow(slice: &[u8]) -> Result<Flow, ErrorInfo> {
        match start_parsing(Span::new(CompleteByteSlice(slice))) {
            Ok((.., instructions)) => create_flow_from_instructions(instructions),
            Err(e) => match e {
                Err::Error(Context::Code(span, code)) => Err(format_error(
                    Interval {
                        line: span.line,
                        column: span.get_column() as u32,
                    },
                    code,
                    &span.fragment,
                )),
                Err::Failure(Context::Code(span, code)) => Err(format_error(
                    Interval {
                        line: span.line,
                        column: span.get_column() as u32,
                    },
                    code,
                    &span.fragment,
                )),
                Err::Incomplete(..) => Err(ErrorInfo {
                    interval: Interval { line: 0, column: 0 },
                    message: "Incomplete".to_string(),
                }),
            },
        }
    }
}

named!(parse_step<Span, Instruction>, do_parse!(
    ident: comment!(parse_ident) >>
    comment!(tag!(COLON)) >>
    start: get_interval >>
    actions: comment!(parse_root_actions) >>
    end: get_interval >>
    (Instruction {
        instruction_type: InstructionType::NormalStep(ident.ident),
        actions: Expr::Block{
            block_type: BlockType::Step,
            arg: actions,
            range: RangeInterval{start, end}
        }
    })
));

named!(start_parsing<Span, Vec<Instruction> >, exact!(
    do_parse!(
        flow: comment!(many0!(parse_step)) >>
        comment!(eof!()) >>
        (flow)
    )
));
