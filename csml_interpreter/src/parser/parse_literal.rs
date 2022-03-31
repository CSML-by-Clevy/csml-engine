use crate::data::{ast::*, tokens::*, Literal};
use crate::parser::tools::get_string;
use crate::parser::tools::get_tag;
use crate::parser::{parse_comments::comment, tools::get_interval};

use crate::data::primitive::{
    boolean::PrimitiveBoolean, float::PrimitiveFloat, int::PrimitiveInt, null::PrimitiveNull,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{opt, recognize},
    error::{ContextError, ParseError},
    multi::{many0, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn signed_digits<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    recognize(tuple((opt(one_of("+-")), decimal)))(s)
}

fn decimal<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(s)
}

fn parse_integer<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, int) = get_int(s)?;

    let expression = Expr::LitExpr {
        literal: PrimitiveInt::get_literal(int, interval),
        in_in_substring: false,
    };
    Ok((s, expression))
}

fn floating_point<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    alt((
        // Case one: .42
        recognize(tuple((char('.'), decimal))), // Case two: 42.42
        recognize(tuple((
            opt(one_of("+-")),
            decimal,
            preceded(char('.'), decimal),
        ))),
    ))(s)
}

fn parse_float<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, float_raw) = floating_point(s)?;
    let float = float_raw.fragment().parse::<f64>().unwrap_or(0.0);

    let expression = Expr::LitExpr {
        literal: PrimitiveFloat::get_literal(float, interval),
        in_in_substring: false,
    };

    Ok((s, expression))
}

fn parse_number<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    alt((parse_float, parse_integer))(s)
}

fn parse_true<'a, E>(s: Span<'a>) -> IResult<Span<'a>, PrimitiveBoolean, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, _) = tag(TRUE)(s)?;

    Ok((s, PrimitiveBoolean::new(true)))
}

fn parse_false<'a, E>(s: Span<'a>) -> IResult<Span<'a>, PrimitiveBoolean, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, _) = tag(FALSE)(s)?;

    Ok((s, PrimitiveBoolean::new(false)))
}

fn parse_boolean<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, boolean) = alt((parse_true, parse_false))(s)?;

    let primitive = Box::new(boolean);
    let expression = Expr::LitExpr {
        literal: Literal {
            content_type: "boolean".to_owned(),
            primitive,
            additional_info: None,
            interval,
        },
        in_in_substring: false,
    };

    Ok((s, expression))
}

fn parse_null<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, _) = get_tag(name.to_ascii_lowercase(), NULL)(s)?;

    let expression = Expr::LitExpr {
        literal: PrimitiveNull::get_literal(interval),
        in_in_substring: false,
    };

    Ok((s, expression))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn get_int<'a, E>(s: Span<'a>) -> IResult<Span<'a>, i64, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    // map_res(signed_digits, |s: Span| s.fragment().parse::<i64>())(s)
    let (s, raw_digits) = signed_digits(s)?;
    let int = raw_digits.fragment().parse::<i64>().unwrap_or(0);

    Ok((s, int))
}

pub fn parse_literal_expr<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
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
            if s.fragment().len() != 0 {
                Err(Err::Error(nom::error::Error::new(s, ErrorKind::Tag)))
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
