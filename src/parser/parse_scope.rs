use crate::comment;
use crate::parser::{
    ast::*, parse_actions::parse_root_functions, parse_ask_response::parse_ask_response,
    parse_for_loop::parse_for, parse_if::parse_if, tokens::*, tools::*,
};
use nom::*;

named!(pub parse_root_actions<Span, Vec<Expr> >, do_parse!(
    actions: many0!(
        alt!(
            parse_if            |
            parse_for           |
            // wait_for
            parse_root_functions|
            parse_ask_response
        )
    ) >>
    (actions)
));

named!(pub parse_implicit_scope<Span, Vec<Expr>>, do_parse!(
    elem: parse_root_functions >>
    (vec![elem])
));

named!(pub parse_strick_scope<Span, Vec<Expr>>, do_parse!(
    vec: delimited!(
        comment!(parse_l_brace),
        parse_root_actions,
        comment!(parse_r_brace)
    ) >>
    (vec)
));

named!(pub parse_scope<Span, Vec<Expr>>, do_parse!(
    vec: delimited!(
        comment!(tag!(L_BRACE)),
        parse_root_actions,
        comment!(parse_r_brace)
    ) >>
    (vec)
));
