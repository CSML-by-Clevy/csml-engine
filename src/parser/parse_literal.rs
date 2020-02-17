use crate::parser::literal::Literal;
use crate::parser::{
    ast::Expr,
    parse_comments::comment,
    parse_idents::{get_string, get_tag},
    tokens::*,
    tools::get_interval,
};

use crate::primitive::{
    boolean::PrimitiveBoolean, float::PrimitiveFloat, int::PrimitiveInt, null::PrimitiveNull,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{complete, map_res, opt, recognize},
    error::ParseError,
    sequence::{pair, preceded, tuple},
    IResult,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn signed_digits<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span, E>
where
    E: ParseError<Span<'a>>,
{
    recognize(tuple((opt(alt((tag("+"), tag("-")))), digit1)))(s)
}

fn parse_integer<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, int) = get_int(s)?;

    let expression = Expr::LitExpr(PrimitiveInt::get_literal("int", int, interval));

    Ok((s, expression))
}

fn floating_point<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span, E>
where
    E: ParseError<Span<'a>>,
{
    recognize(tuple((signed_digits, complete(pair(tag("."), digit1)))))(s)
}

fn parse_float<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, float) = map_res(floating_point, |s: Span| s.fragment.parse::<f64>())(s)?;

    let expression = Expr::LitExpr(PrimitiveFloat::get_literal("float", float, interval));

    Ok((s, expression))
}

fn parse_number<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    alt((parse_float, parse_integer))(s)
}

fn parse_true<'a, E>(s: Span<'a>) -> IResult<Span<'a>, PrimitiveBoolean, E>
where
    E: ParseError<Span<'a>>,
{
    // let (s, interval) = get_interval(s)?;
    let (s, _) = tag(TRUE)(s)?;

    Ok((s, PrimitiveBoolean::new(true)))
}

fn parse_false<'a, E>(s: Span<'a>) -> IResult<Span<'a>, PrimitiveBoolean, E>
where
    E: ParseError<Span<'a>>,
{
    // let (s, interval) = get_interval(s)?;
    // REFACTO
    let (s, _) = tag(FALSE)(s)?;

    // Ok((s, Literal::boolean(false, position)))
    // REFACTO
    Ok((s, PrimitiveBoolean::new(false)))
}

fn parse_boolean<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, boolean) = alt((parse_true, parse_false))(s)?;

    let primitive = Box::new(boolean);
    let expression = Expr::LitExpr(Literal {
        content_type: "boolean".to_owned(),
        primitive,
        interval,
    });

    Ok((s, expression))
}

fn parse_null<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, _) = get_tag(name, NULL)(s)?;

    let expression = Expr::LitExpr(PrimitiveNull::get_literal("null", interval));

    Ok((s, expression))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn get_int<'a, E>(s: Span<'a>) -> IResult<Span<'a>, i64, E>
where
    E: ParseError<Span<'a>>,
{
    map_res(signed_digits, |s: Span| s.fragment.parse::<i64>())(s)
}

pub fn parse_literal_expr<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    // TODO: span: preceded( comment ,  position!() ?
    preceded(comment, alt((parse_number, parse_boolean, parse_null)))(s)
}

////////////////////////////////////////////////////////////////////////////////
// TEST FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, *};

    pub fn test_literal(s: Span) -> IResult<Span, Expr> {
        let var = parse_literal_expr(s);
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
