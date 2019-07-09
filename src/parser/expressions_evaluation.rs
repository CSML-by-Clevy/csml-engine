use crate::comment;
use crate::parser::{ast::*, parse_basic_expr, tokens::*, tools::*};
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

named!(notequal_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(NOT_EQUAL) >>
    (Infix::NotEqual)
));

named!(equal_operator<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(EQUAL) >>
    (Infix::Equal)
));

named!(parse_match<Span, Infix>, do_parse!(
    // position: position!() >>
    tag!(MATCH) >>
    (Infix::Match)
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
    notequal_operator           |
    equal_operator              |
    greaterthanequal_operator   |
    lessthanequal_operator      |
    greaterthan_operator        |
    lessthan_operator
));

named!(parse_not_operator<Span, Infix>, do_parse!(
    tag!(NOT)   >> 
    (Infix::Not)
));

// ########################################

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

named!(parse_infix_condition<Span, Expr>, alt_complete!(
    parse_infix_expr        |
    parse_arithmetic        |
    parse_condition_group
));

// add Null lieral
named!(parse_postfix_operator<Span, Expr>, do_parse!(
    operator: comment!(parse_not_operator) >>
    expr1: parse_arithmetic >>
    (Expr::InfixExpr(operator, Box::new(expr1.clone()), Box::new(expr1)))
));

named!(parse_infix_expr<Span, Expr>, do_parse!(
    expr1: alt!(parse_postfix_operator | parse_arithmetic) >>
    operator: comment!(parse_infix_operators) >>
    expr2: alt!(parse_postfix_operator | parse_arithmetic) >>
    (Expr::InfixExpr(operator, Box::new(expr1), Box::new(expr2)))
));

// ##################################### Arithmetic Operators
named!(parse_arithmetic<Span, Expr>,  alt!(
    parse_item              |
    parse_basic_expr        |
    parse_condition_group
));

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comment;
    use nom::types::*;

    named!(pub test_expressions<Span, Expr>, exact!(comment!(operator_precedence)));

    #[test]
    fn ok_normal_and() {
        let string = Span::new(CompleteByteSlice("3 && event".as_bytes()));
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_or() {
        let string = Span::new(CompleteByteSlice("3 || event".as_bytes()));
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_comparator() {
        let string = Span::new(CompleteByteSlice("3 == event".as_bytes()));
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_arithmetic() {
        let string = Span::new(CompleteByteSlice("3 + (event - 5) * 8 / 3".as_bytes()));
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_complex_expressio() {
        let string = Span::new(CompleteByteSlice(
            "test && (event || hola) && 4 + 3 - 2 ".as_bytes(),
        ));
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_normal_comparation() {
        let string = Span::new(CompleteByteSlice("test == hola >= event".as_bytes()));
        match test_expressions(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }
}
