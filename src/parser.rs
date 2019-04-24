pub mod ast;

use crate::lexer::token::*;
use ast::Literal::*;
use ast::*;
use nom::*;
use std::io::{Error, ErrorKind, Result};

// ################## Macros

macro_rules! tag_token (
    ($i: expr, $tag:path) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));

            if t1.tok.is_empty() {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                match t1.tok[0] {
                    $tag(_)    => Ok((i1, t1)),
                    _          => Err(Err::Error(error_position!($i, ErrorKind::Count)))
                }
            }
        }
    );
);

macro_rules! tag_token2 (
    ($i: expr, $tag:path) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));

            if t1.tok.is_empty() {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                match t1.tok[0] {
                    $tag       => Ok((i1, t1)),
                    _          => Err(Err::Error(error_position!($i, ErrorKind::Count)))
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
                    Token::Ident(name, _) => Ok((i1, Ident(name))),
                    _ => Err(Err::Error(error_position!($i, ErrorKind::Tag))),
                }
            }
        }
    );
);

macro_rules! parse_sub_ident (
    ($i: expr,) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));
            if t1.tok.is_empty() {
                Err(Err::Error(error_position!($i, ErrorKind::Tag)))
            } else {
                match t1.tok[0].clone() {
                    Token::Ident(name, _)  => {
                        if &name == "ask" || &name == "respond" {
                            Ok((i1, Ident(name)))
                        } else {
                            Err(Err::Error(error_position!($i, ErrorKind::Tag)))
                        }
                    },
                    _                      => Err(Err::Error(error_position!($i, ErrorKind::Tag)))
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
                    Token::Equal(_) => Ok((i1, Infix::Equal)),
                    Token::GreaterThan(_) => Ok((i1, Infix::GreaterThan)),
                    Token::LessThan(_) => Ok((i1, Infix::LessThan)),
                    Token::LessThanEqual(_) => Ok((i1, Infix::LessThanEqual)),
                    Token::GreaterThanEqual(_) => Ok((i1, Infix::GreaterThanEqual)),
                    //tmp values
                    Token::And(_) => Ok((i1, Infix::And)),
                    Token::Or(_) => Ok((i1, Infix::Or)),
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
                    Token::IntLiteral(i, _) => Ok((i1, IntLiteral(i))),
                    Token::BoolLiteral(b, _) => Ok((i1, BoolLiteral(b))),
                    Token::StringLiteral(s, _) => Ok((i1, StringLiteral(s))),
                    _ => Err(Err::Error(error_position!($i, ErrorKind::Tag))),
                }
            }
        }
    );
);

// macro_rules! eat_separator2 (
//   ($i:expr, $arr:expr) => (
//     {
//       use $crate::{FindToken, InputTakeAtPosition};
//       let input = $i;
//       input.split_at_position(|c| !$arr.find_token(c))
//     }
//   );
// );

// named!(pub space, eat_separator([Token::Space, Token::NewLine]));

// macro_rules! sp (
//   ($i:expr, $($args:tt)*) => (
//     {
//       sep!($i, space, $($args)*)
//     }
//   )
// );

// //NOTE: ComplexString
// fn ret_test<'a>(rest: Tokens<'a>, vec: Tokens<'a>) -> IResult<Tokens<'a>, Vec<Expr> > {
//     match parse_complex(vec) {
//         Ok((_, vec))    => Ok((rest, vec)),
//         err             => err
//     }
// }

// //NOTE: ComplexString
// macro_rules! parse_complex_string (
//     ($i: expr,) => (
//         {
//             use std::result::Result::*;
//             use nom::{Err,ErrorKind};

//             let (i1, t1) = try_parse!($i, take!(1));
//             if t1.tok.is_empty() {
//                 Err(Err::Error(error_position!($i, ErrorKind::Tag)))
//             } else {
//                 match t1.tok[0].clone() {
//                     Token::ComplexString(vecs) => {
//                         // let tokens = Tokens::new(&vecs);
//                         // ret_test(i1, tokens)
//                         Ok((i1, vec![Expr::LitExpr(StringLiteral("ComplexString".to_owned()))]))
//                     },
//                     _ => Err(Err::Error(error_position!($i, ErrorKind::Tag))),
//                 }
//             }
//         }
//     );
// );

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
                    Token::ReservedFunc(i, _) => Ok((i1, Ident(i))),
                    _ => Err(Err::Error(error_position!($i, ErrorKind::Tag))),
                }
            }
        }
    );
);

//  ################################ STRING

// fn parse_string(input: Tokens) -> IResult<Tokens, Vec<u8> > {
//     use std::result::Result::*;

//     let (i1, c1) = try_parse!(input, take!(1));
//     // println!("i1 {:?} c1 {:?}", i1, c1);
//     match c1.fragment.as_bytes() {
//         b"\"" => Ok((input, vec![])),
//         c => parse_string(i1).map(|(slice, done)| {
//                 // println!("slice {:?}, done {:?}", slice, done);
//                 (slice, concat_slice_vec(c, done))
//             }
//         ),
//     }
// }

// fn concat_slice_vec(c: &[u8], done: Vec<u8>) -> Vec<u8> {
//     let mut new_vec = c.to_vec();
//     new_vec.extend(&done);
//     new_vec
// }

// fn convert_vec_utf8(v: Vec<u8>) -> Result<String, Utf8Error> {
//     let slice = v.as_slice();
//     str::from_utf8(slice).map(|s| s.to_owned())
// }

// named!(to_string<Tokens, String>, do_parse!(
//     )
// );

// named!(string<Tokens, String>, do_parse!(
//         test: delimited!(
//             tag_token!(Token::DoubleQuote),
//             many0!(parse_steps),
//             // map_res!(parse_string, convert_vec_utf8),
//             tag_token!(Token::DoubleQuote)
//         ) >>
//         (test)
//     )
// );
// ################################ Complex Literal

named!(parse_complex_string<Tokens, Expr>, do_parse!(
    vec: delimited!(
        tag_token2!(Token::L2Brace), many0!(parse_var_expr), tag_token2!(Token::R2Brace)
    ) >>
    (Expr::ComplexLiteral(vec))
));

// ################################ FUNC
named!(parse_remember<Tokens, Expr>, do_parse!(
    tag_token!(Token::Remember) >>
    name: parse_ident!() >>
    tag_token!(Token::Assign) >>
    var: parse_var_expr >>
    (Expr::Remember(name, Box::new(var)))
));

named!(parse_goto<Tokens, Expr>, do_parse!(
    tag_token!(Token::Goto) >>
    label: parse_ident!() >>
    (Expr::Goto(label))
));

named!(parse_f<Tokens, Expr>, do_parse!(
    ident: parse_ident!() >>
    vec: parse_expr_group >>
    (Expr::Action{builtin: ident, args: Box::new(vec) })
));

named!(parse_reserved<Tokens, Expr>, do_parse!(
    action: parse_reservedfunc!() >>
    arg: alt!(
        parse_f |
        parse_var_expr
    ) >>
    (Expr::Reserved{fun: action, arg: Box::new(arg)})
));

named!(parse_reserved_empty<Tokens, Expr>, do_parse!(
    action: parse_reservedfunc!() >>
    (Expr::Reserved{fun: action, arg: Box::new(Expr::Empty)})
));

named!(parse_infix_expr<Tokens, Expr>, do_parse!(
    lit1: alt!(
            parse_vec_condition |
            parse_var_expr
    ) >>
    eq: eq_parsers!() >>
    lit2: alt!(
            parse_vec_condition |
            parse_var_expr
    ) >>
    (Expr::InfixExpr(eq, Box::new(lit1), Box::new(lit2)))
));

named!(parse_vec_condition<Tokens, Expr >, do_parse!(
    start_vec: delimited!(
        tag_token!(Token::LParen), parse_condition, tag_token!(Token::RParen)
    ) >>
    (start_vec)
));

named!(parse_condition<Tokens, Expr >, do_parse!(
    condition: alt!(
            parse_infix_expr |
            parse_var_expr
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
                parse_sublabel |
                parse_reserved |
                parse_goto |
                parse_remember |
                parse_if |
                parse_reserved_empty
            )
        ) >>
        (res)
    )
);

named!(parse_sublabel<Tokens, Expr>,
    do_parse!(
        ident: parse_sub_ident!() >>
        tag_token!(Token::Colon) >>
        block: parse_actions >>
        (Expr::Reserved{fun: ident, arg: Box::new(Expr::VecExpr(block)) } )
    )
);

named!(parse_label<Tokens, FlowTypes>,
    do_parse!(
        ident: parse_ident!() >>
        tag_token!(Token::Colon) >>
        block: parse_actions >>
        (FlowTypes::Block(Step{label: ident, actions: block} ))
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

named!(parse_function<Tokens, Expr>, do_parse!(
        ident: parse_ident!() >>
        expr: delimited!(
            tag_token!(Token::LParen), parse_var_expr, tag_token!(Token::RParen)
        ) >>
        (Expr::FunctionExpr(ident, Box::new(expr)))
    )
);

named!(parse_builderexpr<Tokens, Expr>, do_parse!(
    exp1: alt!(
        parse_identexpr |
        parse_literalexpr
    ) >>
    tag_token!(Token::Dot) >>
    exp2: alt!(
        parse_function |
        parse_var_expr
    ) >>
    (Expr::BuilderExpr(Box::new(exp1), Box::new(exp2)))
));

named!(parse_var_expr<Tokens, Expr>, alt!(
        parse_builderexpr   |
        parse_identexpr     |
        parse_literalexpr   |
        parse_vec           |
        parse_complex_string
    )
);

named!(get_exp<Tokens, Expr>, do_parse!(
    tag_token!(Token::Comma) >>
    val: parse_var_expr >>
    (val)
    )
);

named!(parse_expr_group<Tokens, Expr >, do_parse!(
        vec: delimited!(
            tag_token!(Token::LParen), get_vec, tag_token!(Token::RParen)
        ) >>
        (Expr::VecExpr(vec))
    )
);

named!(get_vec<Tokens, Vec<Expr> >, do_parse!(
    res: many1!(
        alt!(
            parse_var_expr |
            get_exp |
            parse_expr_group
        )
    ) >>
    (res)
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

named!(parse_start_flow<Tokens, FlowTypes>,
    do_parse!(
        tag_token!(Token::Flow) >>
        ident: parse_ident!() >>
        start_vec: delimited!(
            tag_token!(Token::LParen), get_vec, tag_token!(Token::RParen)
        ) >>
        (FlowTypes::FlowStarter{ident: ident, list: start_vec})
    )
);

named!(parse_steps<Tokens, FlowTypes>, alt_complete!(
        parse_label |
        parse_start_flow
    )
);

named!(parse_complex<Tokens, Vec<Expr> >,
    do_parse!(
        prog: many0!(parse_var_expr) >>
        (prog)
    )
);

//NOTE: test for WS in parser
// named!(consume_ws<Tokens, bool>,
//     do_parse!(
//         opt!(
//             many0!(
//                 alt!(
//                     tag_token2!(Token::Space) |
//                     tag_token2!(Token::NewLine)
//                 )
//             )
//         ) >>
//         (true)
//     )
// );

named!(parse_program<Tokens, Vec<FlowTypes> >,
    do_parse!(
        prog: many0!(parse_steps) >>
        tag_token2!(Token::EOF) >>
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
                            FlowTypes::Block(step)              => flow.steps.push(step.clone()),
                            FlowTypes::FlowStarter{list, ..}    => flow.accept = list.clone() // replace accept if there are more than one 
                    }
                }
                Ok(flow)
            },
            Err(e) => {
                // TODO: find error type
                println!("error at PARSER {:?}", e);
                Err(Error::new(ErrorKind::Other, "Error at parsing"))
            }
        }
    }
}
