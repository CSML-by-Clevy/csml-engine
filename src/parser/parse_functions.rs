use crate::comment;
use crate::parser::{
    ParserErrorType,
    parse_expr_list, 
    parse_var_expr,
    parse_as_variable,
    ast::*, 
    tokens::*, 
    parse_ident::parse_ident, 
    parse_import::parse_import,
    GotoType
};
use nom::*;

named!(pub parse_assignation<Span, Expr>, do_parse!(
    name: parse_ident >>
    comment!(tag!(ASSIGN)) >>
    expr: complete!(alt!(parse_as_variable | parse_var_expr)) >>
    (Expr::FunctionExpr(ReservedFunction::Assign(name, Box::new(expr))))
));

named!(get_step<Span, GotoType>, do_parse!(
    comment!(tag!(STEP)) >>
    (GotoType::Step)
));

named!(get_file<Span, GotoType>, do_parse!(
    comment!(tag!(FILE)) >>
    (GotoType::File)
));

named!(get_default<Span, GotoType>, do_parse!(
    (GotoType::Step)
));

named!(parse_goto<Span, Expr>, do_parse!(
    comment!(tag!(GOTO)) >>
    goto_type: alt!(get_step | get_file | get_default) >>
    // expr: complete!(parse_var_expr) >>
    name: return_error!(
        nom::ErrorKind::Custom(ParserErrorType::GotoStepError as u32),
        parse_ident
    ) >>
    (Expr::FunctionExpr(ReservedFunction::Goto(goto_type, name)))
));

named!(parse_say<Span, Expr>, do_parse!(
    comment!(tag!(SAY)) >>
    expr: complete!(alt!(parse_as_variable | parse_var_expr)) >>
    (Expr::FunctionExpr(ReservedFunction::Say(Box::new(expr))))
));

named!(parse_use<Span, Expr>, do_parse!(
    comment!(tag!(USE)) >>
    expr: complete!(alt!(parse_as_variable | parse_var_expr)) >>
    (Expr::FunctionExpr(ReservedFunction::Use(Box::new(expr))))
));

named!(parse_remember<Span, Expr>, do_parse!(
    comment!(tag!(REMEMBER)) >>
    ident: comment!(complete!(parse_ident)) >>
    return_error!(
        nom::ErrorKind::Custom(ParserErrorType::AssignError as u32),
        comment!(tag!(ASSIGN))
    ) >>
    expr: complete!(alt!(parse_as_variable | parse_var_expr)) >>
    (Expr::FunctionExpr(ReservedFunction::Remember(ident, Box::new(expr))))
));

named!(pub parse_functions<Span, Expr>, do_parse!(
    name: parse_ident >>
    expr: parse_expr_list >>
    (Expr::FunctionExpr(ReservedFunction::Normal(name, Box::new(expr))))
));

//  RETRY
named!(pub parse_root_functions<Span, Expr>, do_parse!(
    reserved_function: alt!(
        parse_remember          |
        parse_import            |
        parse_goto              |
        parse_say               |
        parse_use
    ) >>
    (reserved_function)
));