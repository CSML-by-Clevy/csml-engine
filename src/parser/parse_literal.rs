use crate::parser::{
    ast::Expr,
    literal::Literal,
    parse_comments::comment,
    tokens::{Span, FALSE, TRUE},
    tools::get_interval, //complete_byte_slice_str_from_utf8, complete_str_from_str,
};
use nom::{
    branch::alt,
    bytes::complete::tag, // take_until, take_till1
    // multi::many0,
    // sequence::delimited,
    character::complete::digit1,
    combinator::{complete, opt},
    combinator::{map_res, recognize},
    error::ParseError,
    sequence::pair,
    sequence::preceded,
    sequence::tuple,
    IResult,
};

pub fn signed_digits<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span, E> {
    recognize(tuple((opt(alt((tag("+"), tag("-")))), digit1)))(s)
}

pub fn get_int<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, i64, E> {
    map_res(signed_digits, |s: Span| s.fragment.parse::<i64>())(s)
}

pub fn parse_integer<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, position) = get_interval(s)?;
    let (s, int) = get_int(s)?;
    Ok((s, Expr::LitExpr(Literal::int(int, position))))
}

pub fn floating_point<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Span, E> {
    recognize(tuple((signed_digits, complete(pair(tag("."), digit1)))))(s)
}

pub fn parse_float<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, position) = get_interval(s)?;
    let (s, float) = map_res(floating_point, |s: Span| s.fragment.parse::<f64>())(s)?;
    Ok((s, Expr::LitExpr(Literal::float(float, position))))
}

pub fn parse_true<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Literal, E> {
    let (s, position) = get_interval(s)?;
    let (s, _) = tag(TRUE)(s)?;
    Ok((s, Literal::boolean(true, position)))
}

pub fn parse_false<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Literal, E> {
    let (s, position) = get_interval(s)?;
    let (s, _) = tag(FALSE)(s)?;
    Ok((s, Literal::boolean(false, position)))
}

pub fn parse_boolean<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, boolean) = alt((parse_true, parse_false))(s)?;
    Ok((s, Expr::LitExpr(boolean)))
}

pub fn parse_literalexpr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    // TODO: span: preceded( comment ,  position!() ?
    preceded(comment, alt((parse_float, parse_integer, parse_boolean)))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, *};

    pub fn test_literal<'a>(s: Span<'a>) -> IResult<Span<'a>, Expr> {
        let var = parse_literalexpr(s);
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
    fn ok_int() {
        let string = Span::new(" +42");
        match test_literal(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_float() {
        let string = Span::new(" -42.42");
        match test_literal(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_bool() {
        let string = Span::new(" true");
        match test_literal(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_sign() {
        let string = Span::new(" +++++4");
        match test_literal(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_float1() {
        let string = Span::new(" 2.2.2");
        match test_literal(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_float2() {
        let string = Span::new(" 3,2 ");
        match test_literal(string) {
            Ok(ok) => panic!("need to fail {:?}", ok),
            Err(..) => {}
        }
    }
}
