pub mod ast;

use crate::lexer::token::*;
use ast::Literal::*;
use ast::*;
use nom::*;
use nom::{Err, ErrorKind as NomError};
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

// ################################ Complex Literal

named!(parse_complex_string<Tokens, Expr>, do_parse!(
    vec: delimited!(
        tag_token2!(Token::L2Brace), many0!(parse_var_expr), tag_token2!(Token::R2Brace)
    ) >>
    (Expr::ComplexLiteral(vec))
));

// ################################ FUNC

// ###################################### FUNC IMPORT

named!(parse_import_opt<Tokens, (Option<Ident>, Option<Ident>, Option<Ident>)>, do_parse!(
    step_name: opt!(
        do_parse!(
            tag_token!(Token::Step) >>
            name: parse_ident!() >>
            (name)
        )
    ) >>
    as_name: opt!(
        do_parse!(
            tag_token!(Token::As) >>
            name: parse_ident!() >>
            (name)
        )
    ) >>
    file_path: opt!(
        do_parse!(
            tag_token!(Token::FromFile) >>
            file_path: parse_ident!() >>
            (file_path)
        )
    ) >>
    ((step_name, as_name, file_path))
));

fn gen_function_expr(name: &str, expr: Expr) -> Expr {
    Expr::FunctionExpr(Ident(name.to_owned()), Box::new(expr))
}

fn gen_builder_expr(expr1: Expr, expr2: Expr) -> Expr {
    Expr::BuilderExpr(Box::new(expr1), Box::new(expr2))
}

fn format_step_options(step_name: Ident, as_name: Option<Ident>, file_path: Option<Ident>) -> Expr{
    match (as_name, file_path) {
        (Some(name), Some(file))    => {
            gen_builder_expr(
                gen_function_expr("step", 
                    gen_function_expr("as", Expr::IdentExpr(name))
                ),
                gen_function_expr("file", Expr::IdentExpr(file))
            )
        },
        (Some(name), None)          => {
            gen_function_expr("step", 
                gen_function_expr("as", Expr::IdentExpr(name))
            )
        },
        (None, Some(file))          => {
            gen_builder_expr(
                gen_function_expr("step", Expr::IdentExpr(step_name)),
                gen_function_expr("file", Expr::IdentExpr(file))
            )
        },
        (None, None)                => gen_function_expr("step", Expr::IdentExpr(step_name)),
    }
}

//OK: nom Custom error handling Example
fn format_import_opt(tokens: Tokens) -> IResult<Tokens , Expr> {
    match parse_import_opt(tokens) {
        Ok((_, (Some(step), as_name, file_path)))   => Ok((tokens, format_step_options(step, as_name, file_path))),
        Ok((_, (None, None, Some(file_path))))      => Ok((tokens, gen_function_expr("file", Expr::IdentExpr(file_path)))),
        Err(e)                                      => Err(e),
        _                                           => Err(Err::Failure(Context::Code(tokens, NomError::Custom(42)))),
    }
}

named!(parse_import_from<Tokens, Expr>, do_parse!(
    expr: format_import_opt >>
    (expr)
));

named!(parse_import<Tokens, Expr>, do_parse!(
    tag_token!(Token::Import) >>
    name: parse_import_from >>
    (Expr::FunctionExpr(Ident("import".to_owned()), Box::new(name)))
));

// ############################################### FUNC IMPORT

named!(parse_assign<Tokens, Expr>, do_parse!(
    name: parse_ident!() >>
    tag_token!(Token::Assign) >>
    var: parse_var_expr >>
    (Expr::Assign(name, Box::new(var)))
));

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
    vec: parse_vec_exp >>
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
                parse_sublabel  |
                parse_reserved  |
                parse_goto      | // tmp
                parse_remember  | // tmp
                parse_assign    | // tmp
                parse_if        |
                parse_reserved_empty
            )
        ) >>
        (res)
    )
);

named!(parse_sublabel<Tokens, Expr>,
    do_parse!(
        ident: parse_ident!() >>
        block: parse_block >>
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
        expr: parse_vec_exp >>
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
        parse_assign        | // tmp

        parse_builderexpr   |
        parse_identexpr     |
        parse_literalexpr   |
        parse_complex_string
    )
);

named!(get_exp<Tokens, Expr>, do_parse!(
    tag_token!(Token::Comma) >>
    val: parse_var_expr >>
    (val)
    )
);

named!(get_vec<Tokens, Vec<Expr> >, do_parse!(
    res: many1!(
        alt!(
            parse_f         |
            parse_vec_exp   |

            get_exp         |
            parse_var_expr
        )
    ) >>
    (res)
    )
);

named!(parse_block<Tokens, Vec<Expr> >, do_parse!(
    block: delimited!(
        tag_token!(Token::LBrace), parse_actions, tag_token!(Token::RBrace)
    ) >>
    (block)
));

named!(parse_vec<Tokens, Vec<Expr> >, do_parse!(
        vec: alt!(
            delimited!(
                tag_token!(Token::LParen), get_vec, tag_token!(Token::RParen)
            ) |
            delimited!(
                tag_token!(Token::LBracket), get_vec, tag_token!(Token::RBracket)
            )
        ) >>
        (vec)
    )
);

named!(parse_vec_exp<Tokens, Expr >, do_parse!(
        vec: parse_vec >>
        (Expr::VecExpr(vec))
    )
);

named!(parse_start_flow<Tokens, FlowTypes>,
    do_parse!(
        tag_token!(Token::Flow) >>
        ident: parse_ident!() >>
        start_vec: parse_vec >>
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
                Err(Error::new(ErrorKind::Other, format!("Error at parsing: {:?}", e)))
            }
        }
    }
}
