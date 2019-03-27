pub mod ast;

use crate::lexer::token::*;
use ast::Literal::*;
use ast::*;
use nom::*;
use std::io::{Error, ErrorKind, Result};

// ################## Macros

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

macro_rules! eq_parsers (
    ($i: expr,) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));
            if t1.tok.is_empty() {
                Err(Err::Error(error_position!($i, ErrorKind::Tag)))
            } else {
                match t1.tok[0].clone() {
                    Token::Equal => Ok((i1, Infix::Equal)),
                    Token::GreaterThan => Ok((i1, Infix::GreaterThan)),
                    Token::LessThan => Ok((i1, Infix::LessThan)),
                    Token::LessThanEqual => Ok((i1, Infix::LessThanEqual)),
                    Token::GreaterThanEqual => Ok((i1, Infix::GreaterThanEqual)),
                    //tmp values
                    Token::And => Ok((i1, Infix::And)),
                    Token::Or => Ok((i1, Infix::Or)),
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

macro_rules! parse_reservedfunc (
    ($i: expr,) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));
            if t1.tok.is_empty() {
                Err(Err::Error(error_position!($i, ErrorKind::Tag)))
            } else {
                match t1.tok[0].clone() {
                    Token::ReservedFunc(i) => Ok((i1, Ident(i))),
                    _ => Err(Err::Error(error_position!($i, ErrorKind::Tag))),
                }
            }
        }
    );
);

// ################################ FUNC

named!(parse_goto<Tokens, Expr>, do_parse!(
    tag_token!(Token::Goto) >>
    label: parse_ident!() >>
    (Expr::Goto(label))
));

named!(parse_f<Tokens, Expr>, do_parse!(
    ident: parse_ident!() >>
    vec: vectorice >>
    (Expr::Action{builtin: ident, args: Box::new(vec) })
));

named!(parse_reserved<Tokens, Expr>, do_parse!(
    action: parse_reservedfunc!() >>
    arg: alt!(
        do_parse!(
            block: parse_block >>
            (Expr::VecExpr(block))
        ) |
        parse_f |
        parse_literalexpr
    ) >>
    (Expr::Reserved{fun: action, arg: Box::new(arg)})
));

named!(parse_reserved_empty<Tokens, Expr>, do_parse!(
    action: parse_reservedfunc!() >>
    (Expr::Reserved{fun: action, arg : Box::new(Expr::Empty)})
));

named!(parse_infixexpr<Tokens, Expr>, do_parse!(
    lit: alt!(
            parse_vec_condition |
            parse_literalexpr |
            parse_identexpr
    ) >>
    eq: eq_parsers!() >>
    lit2: alt!(
            parse_vec_condition |
            parse_literalexpr |
            parse_identexpr
    ) >>
    (Expr::InfixExpr(eq, Box::new(lit), Box::new(lit2)))
));

named!(parse_vec_condition<Tokens, Expr >, do_parse!(
    start_vec: delimited!(
            tag_token!(Token::LParen), parse_condition, tag_token!(Token::RParen)
    ) >>
    (start_vec)
));

named!(parse_condition<Tokens, Expr >, do_parse!(
    condition: alt!(
            parse_infixexpr |
            parse_literalexpr |
            parse_identexpr
    ) >>
    (condition)
));

named!(parse_block<Tokens, Vec<Expr> >, do_parse!(
    block: delimited!(
        tag_token!(Token::LBrace), parse_actions, tag_token!(Token::RBrace)
    ) >>
    (block)
));

named!(parse_if<Tokens, Expr>, do_parse!(
    tag_token!(Token::If) >>
    cond: parse_condition >>
    block: parse_block >>
    (Expr::IfExpr{cond: Box::new(cond), consequence: block})
));

named!(parse_actions<Tokens, Vec<Expr> >,
    do_parse!(
        res: many0!(
            alt!(
                parse_reserved |
                parse_goto |
                parse_if |
                parse_reserved_empty
            )
        )
        >> (res)
    )
);

named!(parse_label<Tokens, Step>,
    do_parse!(
        ident: parse_ident!() >>
        tag_token!(Token::Colon) >>
        block: parse_actions >>
        (Step::Block{ label: ident, actions: block})
    )
);

// ################ pars_to

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
        parse_identexpr |
        parse_literalexpr|
        parse_vec
        // pars_if |
        // parse_action
    )
);

named!(get_exp<Tokens, Expr>, do_parse!(
    tag_token!(Token::Comma) >>
    val: parse_exp >>
    (val)
    )
);

named!(vectorice<Tokens, Expr >, do_parse!(
        vec: delimited!(
            tag_token!(Token::LParen), get_vec, tag_token!(Token::RParen)
        ) >>
        (Expr::VecExpr(vec))
    )
);

named!(get_vec<Tokens, Vec<Expr> >, do_parse!(
    res: many1!(
        alt!(
            parse_exp |
            get_exp |
            vectorice
        )
    )
    >> (res)
    )
);

named!(parse_vec<Tokens, Expr >, do_parse!(
    start_vec: alt!(
        delimited!(
            tag_token!(Token::LParen), get_vec, tag_token!(Token::RParen)
        ) |
        delimited!(
            tag_token!(Token::LBracket), get_vec, tag_token!(Token::RBracket)
        )
    ) >>
    (Expr::VecExpr(start_vec))
));

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

named!(parse_steps<Tokens, Step>, alt_complete!(
        parse_label |
        parse_start_flow
    )
);

named!(parse_program<Tokens, Vec<Step> >,
    do_parse!(
        prog: many0!(parse_steps) >>
        tag_token!(Token::EOF) >>
        (prog)
    )
);

pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: Tokens) -> Result<Flow> {
        let mut flow = Flow{accept: vec![], steps: vec![]};
        // TODO: no use of CLONE and check if there are multiple accepts in flow
        match parse_program(tokens) {
            Ok((_, ast)) => {
                for elem in ast.iter() {
                    match elem {
                            Step::Block{..} => flow.steps.push(elem.clone()),
                            Step::FlowStarter{ident: _ , list} => flow.accept = list.clone()
                    }
                }
                Ok(flow)
            },
            Err(e) => {
                // TODO: find error type
                Err(Error::new(ErrorKind::Other, "Error at parsing"))
            }
        }
    }
}
