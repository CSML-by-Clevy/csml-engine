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

use crate::error_format::CustomError;
use ast::*;
use parse_comments::comment;
use parse_ident::parse_ident;
use parse_scope::parse_root_actions;
use tokens::*;
use tools::*;

use nom::error::ParseError;
use nom::{bytes::complete::tag, multi::many0, sequence::preceded, Err, *};
use std::collections::HashMap;

fn create_flow_from_instructions<'a>(instructions: Vec<Instruction>) -> Result<Flow, String> {
    let mut elem = instructions.iter();
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
    })
}

pub struct Parser;

impl Parser {
    pub fn parse_flow<'a>(slice: &'a str) -> Result<Flow, String> {
        match start_parsing::<CustomError<Span<'a>>>(Span::new(slice)) {
            Ok((.., instructions)) => match create_flow_from_instructions(instructions) {
                Ok(val) => Ok(val),
                Err(error) => Err(error),
            },
            Err(e) => match e {
                Err::Error(err) | Err::Failure(err) => Err(err.error),
                Err::Incomplete(_err) => unimplemented!(),
            },
        }
    }
}

// preceded(comment, )(s)?;
fn parse_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Instruction, E> {
    let (s, ident) = preceded(comment, parse_ident)(s)?;
    let (s, _) = preceded(comment, tag(COLON))(s)?;
    let (s, start) = get_interval(s)?;
    let (s, actions) = preceded(comment, parse_root_actions)(s)?;
    let (s, end) = get_interval(s)?;

    Ok((
        s,
        Instruction {
            instruction_type: InstructionType::NormalStep(ident.ident),
            actions: Expr::Block {
                block_type: BlockType::Step,
                arg: actions,
                range: RangeInterval { start, end },
            },
        },
    ))
}

// named!(start_parsing<Span, Vec<Instruction> >, exact!(
//     do_parse!(
//         flow: comment!(many0!(parse_step)) >>
//         comment!(eof!()) >>
//         (flow)
//     )
// ));
fn start_parsing<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<Instruction>, E> {
    // add comment
    let (s, flow) = many0(parse_step)(s)?;
    //check end of file;
    Ok((s, flow))
}
