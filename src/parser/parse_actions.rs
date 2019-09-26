use crate::comment;
use crate::parser::{
    ast::*,
    parse_ident::parse_ident,
    parse_import::parse_import,
    parse_var_types::{parse_as_variable, parse_expr_list, parse_var_expr},
    tokens::*,
    tools::get_interval,
    GotoType, ParserErrorType,
};
use nom::*;

named!(pub parse_assignation<Span, Expr>, do_parse!(
    name: parse_ident >>
    comment!(tag!(ASSIGN)) >>
    expr: complete!(alt!(parse_as_variable | parse_var_expr)) >>
    (Expr::ObjectExpr(ObjectType::Assign(name, Box::new(expr))))
));

named!(get_step<Span, GotoType>, do_parse!(
    comment!(tag!(STEP)) >>
    (GotoType::Step)
));

named!(get_sub_step<Span, GotoType>, do_parse!(
    comment!(tag!("@")) >>
    (GotoType::SubStep)
));

named!(get_flow<Span, GotoType>, do_parse!(
    comment!(tag!(FLOW)) >>
    (GotoType::Flow)
));

named!(get_default<Span, GotoType>, do_parse!(
    (GotoType::Step)
));

named!(parse_goto<Span, Expr>, do_parse!(
    comment!(tag!(GOTO)) >>
    goto_type: alt!(get_step | get_flow | get_sub_step | get_default) >>
    name: return_error!(
        nom::ErrorKind::Custom(ParserErrorType::GotoStepError as u32),
        parse_ident
    ) >>
    (Expr::ObjectExpr(ObjectType::Goto(goto_type, name)))
));

named!(parse_say<Span, Expr>, do_parse!(
    comment!(tag!(SAY)) >>
    expr: complete!(alt!(parse_as_variable | parse_var_expr)) >>
    (Expr::ObjectExpr(ObjectType::Say(Box::new(expr))))
));

named!(parse_use<Span, Expr>, do_parse!(
    comment!(tag!(USE)) >>
    expr: complete!(alt!(parse_as_variable | parse_var_expr)) >>
    (Expr::ObjectExpr(ObjectType::Use(Box::new(expr))))
));

named!(pub parse_sub_step<Span, Expr>, do_parse!(
    comment!(tag!("@")) >>
    start: get_interval >>
    ident: comment!(complete!(parse_ident)) >>
    end: get_interval >>
    (Expr::Block{block_type: BlockType::SubStep(ident), arg: vec!(), range: RangeInterval{start, end}})
));

named!(parse_remember<Span, Expr>, do_parse!(
    comment!(tag!(REMEMBER)) >>
    expr: comment!(parse_var_expr) >>
    return_error!(
        nom::ErrorKind::Custom(ParserErrorType::AssignError as u32),
        comment!(tag!(AS))
    ) >>
    ident: comment!(complete!(parse_ident)) >>
    (Expr::ObjectExpr(ObjectType::Remember(ident, Box::new(expr))))
));

named!(pub parse_actions<Span, Expr>, do_parse!(
    name: parse_ident >>
    expr: parse_expr_list >>
    (Expr::ObjectExpr(ObjectType::Normal(name, Box::new(expr))))
));

named!(pub parse_root_functions<Span, Expr>, do_parse!(
    reserved_function: alt!(
        // parse_sub_step  |
        parse_remember  |
        parse_import    |
        parse_goto      |
        parse_say       |
        parse_use
    ) >>
    (reserved_function)
));
