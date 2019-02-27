pub mod ast;

use crate::lexer::token::*;
use ast::Literal::*;
use nom::*;
use ast::*;

macro_rules! tag_token (
    ($i: expr, $tag: expr) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));

            if t1.tok.is_empty() {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                if t1.tok[0] == $tag {
                    Ok((i1, t1))
                } else {
                    Err(Err::Error(error_position!($i, ErrorKind::Count)))
                }
            }
        }
    );
);

macro_rules! parse_ident (
    ($i: expr,) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));
            if t1.tok.is_empty() {
                Err(Err::Error(error_position!($i, ErrorKind::Tag)))
            } else {
                match t1.tok[0].clone() {
                    Token::Ident(name) => Ok((i1, Ident(name))),
                    _ => Err(Err::Error(error_position!($i, ErrorKind::Tag))),
                }
            }
        }
    );
);

macro_rules! parse_literal (
    ($i: expr,) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));
            if t1.tok.is_empty() {
                Err(Err::Error(error_position!($i, ErrorKind::Tag)))
            } else {
                match t1.tok[0].clone() {
                    Token::IntLiteral(i) => Ok((i1, IntLiteral(i))),
                    Token::BoolLiteral(b) => Ok((i1, BoolLiteral(b))),
                    Token::StringLiteral(s) => Ok((i1, StringLiteral(s))),
                    _ => Err(Err::Error(error_position!($i, ErrorKind::Tag))),
                }
            }
        }
    );
);

// macro_rules! parse_token (
//     ($i: expr,) => (
//         {
//             use std::result::Result::*;
//             use nom::{Err,ErrorKind};

//             let (i1, t1) = try_parse!($i, take!(1));

//             if t1.tok.is_empty() {
//                 Err(Err::Incomplete(Needed::Size(1)))
//             } else {
//                 Ok((i1, Step::NotYet))
//             }
//         }
//     );
// );

named!(parse_label<Tokens, Step>,
    do_parse!(
        ident: parse_ident!() >>
        tag_token!(Token::Colon) >>
        block: parse_actions >>
        (Step::Block { label: ident, actions: block})
    )
);

named!(parse_actions<Tokens, Vec<Expr> >,
    do_parse!(
        // test: vec![] >>
        (vec![])
    )
);

named!(parse_identexpr<Tokens, Expr>, do_parse!(
        ident: parse_ident!() >>
        (Expr::IdentExpr(ident))
    )
);

named!(parse_literalexpr<Tokens, Expr>, do_parse!(
        literal: parse_literal!() >>
        (Expr::LitExpr(literal))
    )
);

named!(parse_exp<Tokens, Expr>, alt!(
        // pars_if |
        // parse_action
        parse_identexpr |
        parse_literalexpr
    )
);

named!(get_exp<Tokens, Expr>, do_parse!(
    tag_token!(Token::Comma) >>
    val: parse_exp >>
    (val)
    )
);

named!(get_vec<Tokens, Vec<Expr> >, do_parse!(
    res: many1!(
        alt!(
            parse_exp |
            get_exp
        )
    )
    >> (res)
    )
);

named!(parse_start_flow<Tokens, Step>,
    do_parse!(
        tag_token!(Token::Flow) >>
        ident: parse_ident!() >>
        start_vec: delimited!(
            tag_token!(Token::LParen), get_vec, tag_token!(Token::RParen)
        ) >>
        (Step::FlowStarter{ident: ident, list: start_vec})
    )
);

// named!(parse_notyet<Tokens, Step>,
//     do_parse!(
//         test: parse_token!() >>
//         // _elem: parse_ident!() >>
//         // _elem2: parse_literal!() >>
//         (test)
//     )
// );

named!(parse_steps<Tokens, Step>, alt_complete!(
    parse_label |
    parse_start_flow
    // parse_notyet
    )
);

named!(parse_program<Tokens, Flow>,
    do_parse!(
        prog: many0!(parse_steps) >>
        tag_token!(Token::EOF) >>
        (prog)
    )
);

pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: Tokens) -> IResult<Tokens, Flow> {
        parse_program(tokens)
    }
}
