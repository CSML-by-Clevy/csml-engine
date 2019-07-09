pub mod ast;
pub mod parse_ident;
pub mod parse_string;

pub mod expressions_evaluation;
pub mod parse_comments;
pub mod parse_functions;
pub mod parse_if;
pub mod parse_import;
pub mod parse_literal;
pub mod tokens;
pub mod tools;

use ast::*;
use crate::comment;
use expressions_evaluation::operator_precedence;
use parse_functions::{parse_assignation, parse_functions, parse_root_functions};
use parse_ident::parse_ident;
use parse_if::parse_if;
use parse_literal::parse_literalexpr;
use parse_string::parse_string;
use tokens::*;
use tools::*;

use nom::types::*;
use nom::{Err, ErrorKind as NomError, *};
use std::collections::HashMap;

// ################# add marco in nom ecosystem

// #[macro_export]
// macro_rules! tag_or_error{
//     ($tag_name:expr) => {
//         {
//             use nom::*;
//             named!(parse_error<Span, Span>, return_error!(
//                 nom::ErrorKind::Custom(102),   // 102
//                 tag!($tag_name)
//             ));
//         }
//     };
// }

// ##################################### Expr

named!(parse_builderexpr<Span, Expr>, do_parse!(
    ident: parse_identexpr >>
    comment!(tag!(DOT)) >>
    exp: alt!(parse_as_variable | parse_var_expr) >>
    (Expr::BuilderExpr(Box::new(ident), Box::new(exp)))
));

named!(parse_identexpr<Span, Expr>, do_parse!(
    indent: parse_ident >>
    (Expr::IdentExpr(indent))
));

named!(get_list<Span, Expr>, do_parse!(
    first_elem: alt!(parse_as_variable | parse_var_expr) >>
    vec: fold_many0!(
        do_parse!(
            comment!(tag!(COMMA)) >>
            expr: alt!(parse_as_variable | parse_var_expr) >>
            (expr)
        ),
        vec![first_elem],
        |mut acc: Vec<_>, item | {
            acc.push(item);
            acc
        }
    ) >>
    (Expr::VecExpr(vec))
));

named!(parse_mandatory_expr_list<Span, Expr>, do_parse!(
    vec: delimited!(
        comment!(parse_l_parentheses),
        get_list,
        comment!(parse_r_parentheses)
    ) >>
    (vec)
));

named!(parse_expr_list<Span, Expr>, do_parse!(
    vec: delimited!(
        comment!(tag!(L_PAREN)),
        get_list,
        comment!(parse_r_parentheses)
    ) >>
    (vec)
));

named!(parse_expr_array<Span, Expr>, do_parse!(
    vec: delimited!(
        comment!(tag!(L_BRACKET)),
        get_list,
        comment!(parse_r_bracket)
    ) >>
    (vec)
));

named!(pub parse_basic_expr<Span, Expr>, comment!( 
    alt!(
        parse_literalexpr       |
        parse_builderexpr       |
        parse_string            |
        parse_identexpr
    )
));

named!(pub parse_var_expr<Span, Expr>, comment!(
    alt!(
        parse_expr_array        |
        parse_assignation       |
        parse_functions         |
        operator_precedence     |
        parse_basic_expr
    )
));

// ################################### As name

named!(pub parse_as_variable<Span, Expr>, do_parse!(
    expr: parse_var_expr >>
    comment!(tag!(AS)) >>
    name: parse_ident >>
    (Expr::FunctionExpr(ReservedFunction::As(name, Box::new(expr))))
));

// ################################### Ask_Response

named!(parse_ask<Span, (Expr, Option<String> )>, do_parse!(
    comment!(tag!(ASK)) >>
    opt: opt!(parse_ident) >>
    block: parse_block >>
    (Expr::Block{block_type: BlockType::Ask, arg: block}, opt)
));

named!(parse_response<Span, Expr>, do_parse!(
    comment!(tag!(RESPONSE)) >>
    block: parse_strick_block >>
    (Expr::Block{block_type: BlockType::Response, arg: block})
));

named!(normal_ask_response<Span, Expr>, do_parse!(
    ask: parse_ask  >>
    response: parse_response >>
    (Expr::Block{block_type: BlockType::AskResponse(ask.1), arg: vec![ask.0, response]})
));

named!(short_ask_response<Span, Expr>, do_parse!(
    comment!(tag!(ASK)) >>
    ident: opt!(parse_ident) >>
    ask: parse_root_functions >>
    response: many0!(parse_root_functions) >>
    (Expr::Block{
        block_type: BlockType::AskResponse(ident),
        arg: vec![
            Expr::Block{block_type: BlockType::Ask, arg: vec![ask]},
            Expr::Block{block_type: BlockType::Response, arg: response},
        ]
    })
));

named!(parse_ask_response<Span, Expr>, alt!(
    normal_ask_response | short_ask_response
));

// ################################### accept

fn parse_accept(input: Span) -> IResult<Span, bool> {
    match parse_ident(input) {
        Ok((span, ref ident)) if ident == ACCEPT => Ok((span, true)),
        _ => Err(Err::Failure(Context::Code(
            input,
            NomError::Custom(ParserErrorType::AcceptError as u32),
        ))),
    }
}

named!(parse_start_flow<Span, Instruction>, do_parse!(
    tag!(FLOW) >>
    comment!(parse_accept) >>
    actions: parse_mandatory_expr_list  >>

    (Instruction { instruction_type: InstructionType::StartFlow(ACCEPT.to_owned()), actions })
));

// ################################### step

named!(parse_actions<Span, Vec<Expr> >, do_parse!(
    actions: many0!(
        alt!(
            parse_if            |
            parse_root_functions|
            parse_ask_response
        )
    ) >>
    (actions)
));

named!(parse_step<Span, Instruction>, do_parse!(
    ident: comment!(parse_ident) >>
    comment!(tag!(COLON)) >>
    actions: comment!(parse_actions) >>
    (Instruction { instruction_type: InstructionType::NormalStep(ident), actions: Expr::Block{block_type: BlockType::Step, arg: actions} } )
));

// ############################## block

named!(pub parse_strick_block<Span, Vec<Expr>>, do_parse!(
    vec: delimited!(
        comment!(parse_l_brace),
        parse_actions,
        comment!(parse_r_brace)
    ) >>
    (vec)
));

named!(pub parse_block<Span, Vec<Expr>>, do_parse!(
    vec: delimited!(
        comment!(tag!(L_BRACE)),
        parse_actions,
        comment!(parse_r_brace)
    ) >>
    (vec)
));

// ################################

named!(parse_blocks<Span, Instruction>, comment!(
    alt!(
        parse_start_flow |
        parse_step
    )
));

named!(start_parsing<Span, Vec<Instruction> >, exact!(
    do_parse!(
        flow: comment!(many0!(parse_blocks)) >>
        comment!(eof!()) >>
        (flow)
    )
));

fn create_flow_from_instructions(instructions: Vec<Instruction>) -> Result<Flow, ErrorInfo> {
    let mut elem = instructions.iter();
    while let Some(val) = elem.next() {
        let elem2 = elem.clone();
        for val2 in elem2 {
            if val.instruction_type == val2.instruction_type {
                return Err(format_error(Index{ line: 0, column: 0}, ErrorKind::Custom(ParserErrorType::StepDuplicateError as u32) ))
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

#[repr(u32)]
pub enum ParserErrorType {
    StepDuplicateError = 0,
    AssignError = 1,
    GotoStepError = 10,
    ImportError = 11,
    ImportStepError = 12,
    AcceptError = 100,
    LeftBraceError = 110,
    RightBraceError = 111,
    LeftParenthesesError = 112,
    RightParenthesesError = 113,
    RightBracketError = 114,
    DoubleQuoteError = 120,
    DoubleBraceError = 130,
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub line: u32,
    pub column: u32,
    pub message: String,
}

fn get_error_message(error_code: ErrorKind) -> String {
    match error_code {
        ErrorKind::Custom(val) if val == ParserErrorType::StepDuplicateError as u32 => {
           "ERROR: Step name already exists".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::AssignError as u32 => {
            "ERROR: Missing = after remember statement".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::GotoStepError as u32 => {
            "ERROR: Missing label name after goto".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::AcceptError as u32 => {
            "ERROR: Flow argument expect Accept identifier".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::LeftBraceError as u32 => {
            "ERROR: Missing start of block { ".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::RightBraceError as u32 => {
            "ERROR: Agruments inside brace bad format or brace missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::LeftParenthesesError as u32 => {
            "ERROR: ( mabe missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::RightParenthesesError as u32 => {
            "ERROR: Agruments inside parentheses bad format or ) missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::RightBracketError as u32 => {
            "ERROR: Agruments inside parentheses bad format or ] missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::DoubleQuoteError as u32 => {
            "ERROR: \" mabe missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::DoubleBraceError as u32 => {
            "ERROR: }} mabe missing".to_string()
        }
        e => e.description().to_owned(),
    }
}

fn format_error(index: Index, error_code: ErrorKind) -> ErrorInfo {
    let message = get_error_message(error_code);

    ErrorInfo {
        line: index.line,
        column: index.column,
        message,
    }
}

pub struct Parser;

impl Parser {
    pub fn parse_flow(slice: &[u8]) -> Result<Flow, ErrorInfo> {
        match start_parsing(Span::new(CompleteByteSlice(slice))) {
            Ok((.., instructions)) => create_flow_from_instructions(instructions),
            Err(e)                 => match e {
                Err::Error(Context::Code(span, code)) => Err(format_error(Index{ line: span.line, column: span.get_column() as u32}, code)),
                Err::Failure(Context::Code(span, code)) => Err(format_error(Index{ line: span.line, column: span.get_column() as u32}, code)),
                Err::Incomplete(..) => Err(ErrorInfo {
                    line: 0,
                    column: 0,
                    message: "Incomplete".to_string(),
                }),
            },
        }
    }
}
