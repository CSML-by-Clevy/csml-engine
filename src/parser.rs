pub mod ast;
pub mod parse_ident;
pub mod parse_string;

pub mod tools;
pub mod tokens;
pub mod parse_comments;
pub mod parse_functions;
pub mod parse_if;

use crate::comment;

use tokens::*;
use ast::*;
use tools::parse_literalexpr;
use parse_ident::parse_ident;
use parse_string::parse_string;
use parse_functions::{parse_root_functions, parse_functions, parse_assignation};
use parse_if::{parse_if, operator_precedence};

use nom::{*, Err, ErrorKind as NomError};
use nom::types::*;
// use nom::{Err, ErrorKind as NomError};
// use nom_locate::position;

use std::collections::HashMap;
// use std::str;

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
    exp: parse_var_expr >>
    (Expr::BuilderExpr(Box::new(ident), Box::new(exp)))
));

named!(parse_identexpr<Span, Expr>, do_parse!(
    indent: parse_ident >>
    (Expr::IdentExpr(indent))
));

named!(get_list<Span, Expr>, do_parse!(
    first_elem: parse_var_expr >>
    vec: fold_many0!(
        do_parse!(
            comment!(tag!(COMMA)) >>
            expr: parse_var_expr >>
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


named!(parse_r_parentheses<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::RightParenthesesError as u32),
    tag!(R_PAREN)
));

named!(parse_l_parentheses<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::LeftParenthesesError as u32),
    tag!(L_PAREN)
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

named!(parse_r_bracket<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::RightBracketError as u32),
    tag!(R_BRACKET)
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

// ################################### Ask_Response

named!(parse_ask<Span, Expr>, do_parse!(
    comment!(tag!(ASK)) >>
    block: parse_block >>
    (Expr::Block{block_type: BlockType::Ask, arg: block})
));

named!(parse_response<Span, Expr>, do_parse!(
    comment!(tag!(RESPONSE)) >>
    block: parse_block >>
    (Expr::Block{block_type: BlockType::Response, arg: block})
));

named!(parse_ask_response<Span, Expr>, do_parse!(
    tuple: permutation!(parse_ask, parse_response) >>
    (Expr::Block{block_type: BlockType::AskResponse, arg: vec![tuple.0, tuple.1]})
));

// ################################### accept

fn parse_accept(input: Span) -> IResult<Span, bool> {
    match parse_ident(input) {
        Ok((span, ref ident)) if ident == ACCEPT => Ok((span, true)),
        _                                        => Err(Err::Failure(Context::Code(input, NomError::Custom(ParserErrorType::AcceptError as u32))))
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

named!(parse_l_brace<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::LeftBraceError as u32),
    tag!(L_BRACE)
));

named!(parse_r_brace<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::RightBraceError as u32),
    tag!(R_BRACE)
));

named!(pub parse_block<Span, Vec<Expr>>, do_parse!(
    vec: delimited!(
        comment!(parse_l_brace),
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

// TODO: check for steps with the same name and return ERROR
fn create_flow_from_instructions(instructions: Vec<Instruction>) -> Flow {
    Flow {
        flow_instructions:
            instructions
                .into_iter()
                .map(|elem| (elem.instruction_type, elem.actions))
                .collect::<HashMap<InstructionType, Expr> >()
    }
}

#[repr(u32)]
pub enum ParserErrorType {
    AssignError             = 1,
    GotoStepError           = 10,
    AcceptError             = 100,
    LeftBraceError          = 110,
    RightBraceError         = 111,
    LeftParenthesesError    = 112,
    RightParenthesesError   = 113,
    RightBracketError       = 114,
    DoubleQuoteError        = 120,
    DoubleBraceError        = 130
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub line: u32,
    pub colon: u32,
    pub message: String,
}

fn get_error_message(error_code: ErrorKind) -> String {
    match error_code {
        ErrorKind::Custom(val) if val == ParserErrorType::AssignError as u32            => "ERROR: Missing = after remember statement".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::GotoStepError as u32          => "ERROR: Missing label name after goto".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::AcceptError as u32            => "ERROR: Flow argument expect Accept identifier".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::LeftBraceError as u32         => "ERROR: Missing start of block { ".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::RightBraceError as u32        => "ERROR: Agruments inside brace bad format or brace missing".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::LeftParenthesesError as u32   => "ERROR: ( mabe missing".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::RightParenthesesError as u32  => "ERROR: Agruments inside parentheses bad format or ) missing".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::RightBracketError as u32      => "ERROR: Agruments inside parentheses bad format or ] missing".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::DoubleQuoteError as u32       => "ERROR: \" mabe missing".to_string(),
        ErrorKind::Custom(val) if val == ParserErrorType::DoubleBraceError as u32       => "ERROR: }} mabe missing".to_string(),
        e                                                                               => e.description().to_owned()
    }
}

fn format_error(e: Span, error_code: ErrorKind) -> ErrorInfo {
    let message = get_error_message(error_code);

    ErrorInfo{line: e.line, colon: e.get_column() as u32, message}
}

pub struct Parser;

impl Parser {
    pub fn parse_flow(slice: &[u8]) -> Result<Flow, ErrorInfo> {
        match start_parsing(Span::new(CompleteByteSlice(slice))) {
            Ok((.., instructions)) => {
                Ok(create_flow_from_instructions(instructions))
            }
            Err(e) => {
                match e {
                    Err::Error(Context::Code(span, code))      => Err(format_error(span, code)),
                    Err::Failure(Context::Code(span, code))    => Err(format_error(span, code)),
                    Err::Incomplete(..)                        => Err(ErrorInfo{line: 0, colon: 0, message: "Incomplete".to_string()})
                }
            },
        }
    }
}
