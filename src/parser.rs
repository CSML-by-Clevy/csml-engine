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

// named!(parse_<Tokens, Expr>, do_parse!(
// ));

named!(parse_goto<Tokens, Expr>, do_parse!(
    tag_token!(Token::Goto) >>
    label: parse_ident!() >>
    (Expr::Goto(label))
));

named!(parse_f<Tokens, Expr>, do_parse!(
    ident: parse_ident!() >>
    vec: delimited!(
        tag_token!(Token::LParen), get_vec, tag_token!(Token::RParen)
    ) >>
    (Expr::Action{fun: ident, arg: vec })
));

named!(parse_reserved<Tokens, Expr>, do_parse!(
    action:  parse_reservedfunc!() >>
    arg: alt!( 
        parse_f |
        parse_literalexpr
     ) >>
    (Expr::Reserved{fun : action , arg : Box::new(arg)})
));

// named!(parse_function<Tokens, Expr>, alt!(
//     )
// );

// A && B && C || D && E
// named!(logique_parser<Tokensm Expr>, do_prase!(
//     vec: many0!(
//         do_parse!(
            
//         )
//     ) >>
// ));

// named!(parse_comparators<Tokens, Expr>,
    // eq_parsers!()
    // alt!(
    // logique_parser
    // )
// );

named!(parse_infixexpr<Tokens, Expr>, do_parse!(
    lit: alt!(
            parse_literalexpr  |
            parse_identexpr
        )
    >>
    eq: eq_parsers!() >>
    (Expr::InfixExpr(eq, Box::new(lit) ))
));

named!(parse_condition<Tokens, Vec<Expr> >, do_parse!(
    vec: many1!(
        alt!(
            parse_infixexpr |
            parse_literalexpr |
            parse_identexpr
        )
    ) >>
    (vec)
));

named!(parse_if<Tokens, Expr>, do_parse!(
    tag_token!(Token::If) >>
    cond: parse_condition >>
    block: delimited!(
        tag_token!(Token::LBrace), parse_actions, tag_token!(Token::RBrace)
    ) >>
    (Expr::IfExpr{cond: cond, consequence: block})
));

named!(parse_actions<Tokens, Vec<Expr> >,
    do_parse!(
        res: many0!(
            alt!(
                parse_reserved |
                parse_goto |
                parse_if
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

//     do_parse!(
//     tag_token!(Token::Flow) >>
//     ident: parse_ident!() >>
//     start_vec: delimited!(
//         tag_token!(Token::LParen), get_vec, tag_token!(Token::RParen)
//     ) >>
//     (Step::FlowStarter{ident: ident, list: start_vec})
// )

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
    // pub fn parse_labels(tokens: Vec<Token>) -> Vec<Token> {
    // }
}
