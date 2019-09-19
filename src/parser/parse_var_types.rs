use nom::*;
use crate::comment;
use crate::error_format::data::ParserErrorType;
use crate::parser::{
    ast::*,
    tools::*,
    tokens::*,
    parse_ident::parse_ident,
    parse_string::parse_string,
    parse_literal::parse_literalexpr,
    expressions_evaluation::operator_precedence,
    parse_actions::{parse_assignation, parse_actions},
};

named!(parse_builderexpr<Span, Expr>, do_parse!(
    ident: parse_identexpr >>
    comment!(tag!(DOT)) >>
    exp: alt!(parse_builderexpr | parse_identexpr) >>
    (Expr::BuilderExpr(Box::new(ident), Box::new(exp)))
));

named!(parse_identexpr<Span, Expr>, do_parse!(
    indent: parse_ident >>
    (Expr::IdentExpr(indent))
));


named!(get_list<Span, Expr>, do_parse!(
    first_elem: alt!(parse_as_variable | parse_var_expr) >>
    start: get_interval >>
    vec: fold_many0!(
        do_parse!(
            comment!(tag!(COMMA)) >>
            expr: alt!(parse_as_variable | parse_var_expr) >>
            (expr)
        ),
        vec![first_elem],
        |mut acc: Vec<_>, item | {
            acc.push(item);
            acc
        }
    ) >>
    end: get_interval >>
    (Expr::VecExpr(vec, RangeInterval{start, end}))
));

named!(get_empty_list<Span, Expr>, do_parse!(
    comment!(tag!(L_PAREN)) >>
    start: get_interval >>
    comment!(parse_r_parentheses) >>
    end: get_interval >>
    (Expr::VecExpr(vec!(), RangeInterval{start, end}))
));

named!(pub parse_expr_list<Span, Expr>, do_parse!(
    vec: alt!(
        delimited!(
            comment!(tag!(L_PAREN)),
            get_list,
            comment!(parse_r_parentheses)
        ) |
        get_empty_list
    ) >>
    (vec)
));

named!(get_empty_array<Span, Expr>, do_parse!(
    comment!(tag!(L_BRACKET)) >>
    start: get_interval >>
    comment!(parse_r_bracket) >>
    end: get_interval >>
    (Expr::VecExpr(vec!(), RangeInterval{start, end}))
));

named!(parse_expr_array<Span, Expr>, do_parse!(
    vec: alt!(
        delimited!(
            comment!(tag!(L_BRACKET)),
            get_list,
            comment!(parse_r_bracket)
        ) |
        get_empty_array
    ) >>
    (vec)
));

named!(pub parse_mandatory_expr_list<Span, Expr>, do_parse!(
    vec: delimited!(
        comment!(parse_l_parentheses),
        get_list,
        comment!(parse_r_parentheses)
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
        parse_actions         |
        operator_precedence     |
        parse_basic_expr
    )
));

pub fn parse_as_basic_variable(span: Span) -> IResult<Span, Expr> {
    let (span, expr) = parse_basic_expr(span)?;
    let (span, smart_lit) = parse_ident(span)?;
    if smart_lit.ident != "as" {
        return Err(Err::Error(
            Context::Code(
                    span,
                    ErrorKind::Custom(ParserErrorType::DoubleBraceError as u32),
            )
        ))
    }
    let (span, name) = parse_ident(span)?;
    (Ok((span, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr))) )))
}

pub fn parse_as_variable(span: Span) -> IResult<Span, Expr> {
    let (span, expr) = parse_var_expr(span)?;
    let (span, smart_lit) = parse_ident(span)?;
    if smart_lit.ident != "as" {
        return Err(Err::Error(
            Context::Code(
                    span,
                    ErrorKind::Custom(ParserErrorType::DoubleBraceError as u32),
            )
        ))
    }
    let (span, name) = parse_ident(span)?;
    (Ok((span, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr))) )))
}
