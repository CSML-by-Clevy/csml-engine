use crate::parser::{
    ast::*, parse_comments::comment, parse_var_types::parse_basic_expr, tokens::*, tools::*,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    error::ParseError,
    multi::fold_many0,
    sequence::{preceded, tuple},
    *,
};

pub fn or_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(OR)(s)?;
    Ok((rest, Infix::Or))
}

pub fn and_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(AND)(s)?;
    Ok((rest, Infix::And))
}

pub fn notequal_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(NOT_EQUAL)(s)?;
    Ok((rest, Infix::NotEqual))
}

pub fn equal_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(EQUAL)(s)?;
    Ok((rest, Infix::Equal))
}

pub fn parse_match<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(MATCH)(s)?;
    Ok((rest, Infix::Match))
}

pub fn greaterthanequal_operator<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(GREATER_THAN_EQUAL)(s)?;
    Ok((rest, Infix::GreaterThanEqual))
}

pub fn lessthanequal_operator<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(LESS_THAN_EQUAL)(s)?;
    Ok((rest, Infix::LessThanEqual))
}

pub fn greaterthan_operator<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(GREATER_THAN)(s)?;
    Ok((rest, Infix::GreaterThan))
}

pub fn lessthan_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(LESS_THAN)(s)?;
    Ok((rest, Infix::LessThan))
}

pub fn parse_infix_operators<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Infix, E> {
    alt((
        notequal_operator,
        parse_match,
        equal_operator,
        greaterthanequal_operator,
        lessthanequal_operator,
        greaterthan_operator,
        lessthan_operator,
    ))(s)
}

fn parse_not_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (rest, ..) = tag(NOT)(s)?;
    Ok((rest, Infix::Not))
}

// ########################################

fn parse_or<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, or_operator)(s)?;
    parse_and_condition(s)
}

pub fn operator_precedence<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, init) = parse_and_condition(s)?;
    fold_many0(parse_or, init, |acc, value: Expr| {
        Expr::InfixExpr(Infix::Or, Box::new(acc), Box::new(value))
    })(s)
}

fn parse_and<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, _) = preceded(comment, and_operator)(s)?;
    parse_infix_condition(s)
}

fn parse_and_condition<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, init) = parse_infix_condition(s)?;
    fold_many0(parse_and, init, |acc, value: Expr| {
        Expr::InfixExpr(Infix::And, Box::new(acc), Box::new(value))
    })(s)
}

fn parse_infix_condition<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    alt((
        parse_infix_expr,
        alt((parse_postfix_operator, parse_arithmetic)),
        parse_condition_group,
    ))(s)
}

fn parse_postfix_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, operator) = preceded(comment, parse_not_operator)(s)?;
    let (s, expr1) = parse_arithmetic(s)?;
    Ok((
        s,
        Expr::InfixExpr(operator, Box::new(expr1.clone()), Box::new(expr1)), //  InfixExpr clone in not operator or create a new expr for not??
    ))
}

fn parse_infix_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, expr1) = alt((parse_postfix_operator, parse_arithmetic))(s)?;
    let (s, operator) = preceded(comment, parse_infix_operators)(s)?;
    let (s, expr2) = alt((parse_postfix_operator, parse_arithmetic))(s)?;

    Ok((
        s,
        Expr::InfixExpr(operator, Box::new(expr1), Box::new(expr2)),
    ))
}

// ##################################### Arithmetic Operators

fn parse_arithmetic<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    alt((parse_item, parse_basic_expr, parse_condition_group))(s)
}

fn adition_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (s, _) = tag(ADITION)(s)?;
    Ok((s, Infix::Adition))
}

fn remainder_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (s, _) = tag(REMAINDER)(s)?;
    Ok((s, Infix::Remainder))
}

fn substraction_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (s, _) = tag(SUBTRACTION)(s)?;
    Ok((s, Infix::Substraction))
}

fn parse_item_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    alt((substraction_operator, adition_operator))(s)
}

fn parse_item<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, init) = parse_term(s)?;
    fold_many0(
        tuple((preceded(comment, parse_item_operator), parse_term)),
        init,
        |acc, v: (Infix, Expr)| Expr::InfixExpr(v.0, Box::new(acc), Box::new(v.1)),
    )(s)
}

fn divide_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (s, _) = tag(DIVIDE)(s)?;
    Ok((s, Infix::Divide))
}

fn multiply_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    let (s, _) = tag(MULTIPLY)(s)?;
    Ok((s, Infix::Multiply))
}

fn parse_term_operator<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Infix, E> {
    alt((divide_operator, multiply_operator, remainder_operator))(s)
}

fn parse_term<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, init) = alt((parse_basic_expr, parse_condition_group))(s)?;
    fold_many0(
        tuple((
            preceded(comment, parse_term_operator),
            alt((parse_basic_expr, parse_condition_group)),
        )),
        init,
        |acc, v: (Infix, Expr)| Expr::InfixExpr(v.0, Box::new(acc), Box::new(v.1)),
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    pub fn test_expressions<'a>(s: Span<'a>) -> IResult<Span<'a>, Expr> {
        let var = preceded(comment, operator_precedence)(s);
        if let Ok((s, v)) = var {
            if s.fragment.len() != 0 {
                Err(Err::Error((s, ErrorKind::Tag)))
            } else {
                Ok((s, v))
            }
        } else {
            var
        }
    }

    #[test]
    fn ok_normal_and() {
        let string = Span::new("3 && event");
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_or() {
        let string = Span::new("3 || event");
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_comparator() {
        let string = Span::new("3 == event");
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_arithmetic() {
        let string = Span::new("3 + (event - 5) * 8 / 3");
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_complex_expressio() {
        let string = Span::new("test && (event || hola) && 4 + 3 - 2");
        match test_expressions(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_normal_comparation() {
        let string = Span::new("test == hola >= event");
        match test_expressions(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }
}
