use crate::comment;
use crate::parser::{tools::*, parse_block, ast::*, tokens::*};
use nom::*;


named!(pub parse_if<Span, Expr>, do_parse!(
    comment!(tag!(IF)) >>
    condition: parse_strict_condition_group >>
    block: comment!(parse_block) >>
    (Expr::IfExpr{cond: Box::new(condition), consequence: block})
));
