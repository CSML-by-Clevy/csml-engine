use crate::comment;
use crate::parser::{ParserErrorType, parse_block, parse_basic_expr, ast::*, tokens::*};
use nom::*;

named!(or_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(OR) >> 
    (Infix::Or)
));

named!(and_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(AND)  >> 
    (Infix::And)
));

// NotEqual,
named!(equal_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(EQUAL) >>
    (Infix::Equal)
));

named!(greaterthanequal_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(GREATER_THAN_EQUAL) >>
    (Infix::GreaterThanEqual)
));

named!(lessthanequal_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(LESS_THAN_EQUAL)  >>
    (Infix::LessThanEqual)
));

named!(greaterthan_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(GREATER_THAN)   >> 
    (Infix::GreaterThan)
));

named!(lessthan_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(LESS_THAN)   >> 
    (Infix::LessThan)
));

named!(parse_infix_operators<Span, Infix>, alt!(
    equal_operator              |
    greaterthanequal_operator   |
    lessthanequal_operator      |
    greaterthan_operator        |
    lessthan_operator
));

named!(parse_and_condition<Span, Expr>, do_parse!(
    init: parse_infix_condition >>
    and_expr: fold_many0!(
        do_parse!(
            comment!(and_operator) >>
            expr: parse_infix_condition >>
            (expr)
        ),
        init,
        |acc, value:Expr| {
            Expr::InfixExpr(Infix::And, Box::new(acc), Box::new(value))
        }
    ) >>
    (and_expr)
));

named!(parse_l_parentheses<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::LeftParenthesesError as u32),
    tag!(L_PAREN)
));

//duplicate rm
named!(parse_r_parentheses<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::RightParenthesesError as u32),
    tag!(R_PAREN)
));

named!(parse_condition_group<Span, Expr>, delimited!(
    comment!(parse_l_parentheses),
    operator_precedence,
    comment!(parse_r_parentheses)
));

named!(pub operator_precedence<Span, Expr>, do_parse!(
        init: parse_and_condition >>
        and_expr: fold_many0!(
            do_parse!(
                comment!(or_operator) >>
                expr: parse_and_condition >>
                (expr)
            ),
            init,
            |acc, value:Expr| {
                Expr::InfixExpr(Infix::Or, Box::new(acc), Box::new(value))
            }
        )
        >> (and_expr)
));

named!(parse_infix_condition<Span, Expr>, alt!(
    parse_infix_expr        |
    parse_arithmetic        |
    parse_condition_group
));

named!(parse_infix_expr<Span, Expr>, do_parse!(
    expr1: parse_arithmetic >>
    operator: comment!(parse_infix_operators) >>
    expr2: parse_arithmetic >>
    (Expr::InfixExpr(operator, Box::new(expr1), Box::new(expr2)))
));

named!(pub parse_if<Span, Expr>, do_parse!(
    comment!(tag!(IF)) >>
    condition: parse_condition_group >>
    block: comment!(parse_block) >>
    (Expr::IfExpr{cond: Box::new(condition), consequence: block})
));

// ##################################### Arithmetic Operators
// factor ->
    // int
    // float
    // ( item )

named!(parse_arithmetic<Span, Expr>,  alt!(
    parse_item              |
    parse_basic_expr        |
    parse_condition_group
));

// item -> term (+ -) term
named!(adition_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(ADITION)  >> 
    (Infix::Adition)
));

named!(substraction_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(SUBTRACTION)  >> 
    (Infix::Substraction)
));

named!(parse_item_operator<Span, Infix>, alt!(
    substraction_operator |
    adition_operator
));

// span: position!() >>
named!(parse_item<Span, Expr>, do_parse!(
    init: parse_term >>
    and_expr: fold_many0!(
        tuple!(
            comment!(parse_item_operator),
            parse_term
        ),
        init,
        |acc, v:(Infix, Expr)| {
            Expr::InfixExpr(v.0, Box::new(acc), Box::new(v.1))
        }
    ) >>
    (and_expr)
));

// term -> factor (* /) factor
named!(divide_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(DIVIDE)  >> 
    (Infix::Divide)
));

named!(multiply_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(MULTIPLY)  >> 
    (Infix::Multiply)
));

named!(parse_term_operator<Span, Infix>, alt!(
    divide_operator |
    multiply_operator
));

named!(parse_term<Span, Expr>, do_parse!(
    init: alt!(parse_basic_expr | parse_condition_group) >>
    and_expr: fold_many0!(
        tuple!(
            comment!(parse_term_operator),
            alt!(parse_basic_expr | parse_condition_group)
        ),
        init,
        |acc, v:(Infix, Expr)| {
            Expr::InfixExpr(v.0, Box::new(acc), Box::new(v.1))
        }
    ) >>
    (and_expr)
));
