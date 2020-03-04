use crate::data::primitive::string::PrimitiveString;
use crate::data::{ast::*, tokens::*};
use crate::parser::operator::parse_operator;
use crate::parser::{parse_comments::comment, tools::get_interval};
use nom::{
    bytes::complete::tag,
    error::{ErrorKind, ParseError},
    multi::many_till,
    sequence::{delimited, preceded},
    *,
};
use std::str;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_2brace<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<Expr>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag(L2_BRACE)(s)?;
    let (s, (vec, _)) = many_till(parse_operator, preceded(comment, tag(R2_BRACE)))(s)?;

    Ok((s, vec))
}

fn parse_brace<'a, E>(input: Span<'a>, mut vec: Vec<Expr>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    match parse_2brace(input) {
        Ok((s, mut exprs)) => {
            vec.append(&mut exprs);

            match parse_complex_string(s) {
                Ok((s2, Expr::ComplexLiteral(mut vec2, range))) => {
                    vec.append(&mut vec2);
                    // TODO: BAD RANGE this is only for test
                    Ok((s2, Expr::ComplexLiteral(vec, range)))
                }
                Ok((s2, expr)) => {
                    if vec.is_empty() {
                        Ok((s2, expr))
                    } else {
                        vec.push(expr);
                        let (s2, p) = get_interval(s2)?;
                        Ok((
                            s2,
                            Expr::ComplexLiteral(vec, RangeInterval { start: p, end: p }),
                        ))
                    }
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

fn get_distance(input: &Span, key_char: &str) -> (Option<usize>, Option<usize>) {
    let distance_to_key = input.find_substring(key_char);
    let distance_double_quote = input.find_substring(DOUBLE_QUOTE);
    (distance_to_key, distance_double_quote)
}

fn parse_complex_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    match get_distance(&s, L2_BRACE) {
        (Some(distance_to_l2brace), Some(distance_double_quote))
            if distance_to_l2brace < distance_double_quote =>
        {
            let (s, val) = s.take_split(distance_to_l2brace);
            let (val, interval) = get_interval(val)?;
            let mut vec = vec![];

            if val.input_len() > 0 {
                let expression =
                    Expr::LitExpr(PrimitiveString::get_literal(val.fragment(), interval));
                vec.push(expression);
            }
            parse_brace(s, vec)
            //  {
            //     Ok((s, vec)) => Ok((s, vec)),
            //     // Err(Err::Failure(e)) => Err(Err::Failure(e)),
            //     Err(_) => Err(Err::Failure(
            //         E::add_context(s, "DoubleQuoteError", E::from_error_kind(s, ErrorKind::Tag))
            //     )),
            // }
        }
        (_, Some(distance_double_quote)) => {
            let (s, val) = s.take_split(distance_double_quote);
            let (val, interval_start) = get_interval(val)?;

            if val.input_len() > 0 {
                let expression =
                    Expr::LitExpr(PrimitiveString::get_literal(val.fragment(), interval_start));

                return Ok((s, expression));
            }

            let (_, interval_end) = get_interval(val)?;
            Ok((
                s,
                Expr::ComplexLiteral(
                    vec![],
                    RangeInterval {
                        start: interval_start,
                        end: interval_end,
                    },
                ),
            ))
        }
        (_, None) => Err(Err::Failure(E::add_context(
            s,
            "DoubleQuoteError",
            E::from_error_kind(s, ErrorKind::Tag),
        ))),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    delimited(tag(DOUBLE_QUOTE), parse_complex_string, tag(DOUBLE_QUOTE))(s)
}

////////////////////////////////////////////////////////////////////////////////
// TEST FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_string(s: Span) -> IResult<Span, Expr> {
        preceded(comment, parse_string)(s)
    }

    #[test]
    fn ok_normal_string() {
        let string = Span::new("\"normal string\"");
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_comment_string() {
        let string = Span::new("    \"normal string\"    /* test */");
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_normal_string_no_right_quote() {
        let string = Span::new(" \"normal string ");
        match test_string(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_normal_string_no_left_quote() {
        let string = Span::new(" normal string\" ");
        match test_string(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn ok_complex_string() {
        let string = Span::new("  \"complex string {{ \"test\" }}\"  ");
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_complex_complex_string() {
        let string = Span::new("  \"complex string {{ \"var {{ \"test\" }}\" }}\"  ");
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_complex_string_no_right_bracket() {
        let string = Span::new("  \"complex string {{ \"  ");
        match test_string(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_complex_string_no_left_bracket() {
        let string = Span::new("  \"complex string  }}\"  ");
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }
}
