use crate::data::primitive::string::PrimitiveString;
use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_failure, *};
use crate::parser::tools::get_interval;

use nom::{bytes::complete::tag, error::ParseError, sequence::delimited, *};

// TODO:
// GOOD ERROR MESSAGE
// GOOD INTERVAL
// WRITE TESTS
// UNCOMMENT OBJECT TEST

////////////////////////////////////////////////////////////////////////////////
// TOOL FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn condition_brace(s: &Span, key: char, c: char, escape: bool, index: usize) -> bool {
    if c == key && escape == false {
        if let Some(c) = s.fragment().chars().nth(index + 1) {
            if c == key {
                return true;
            }
        }
    }

    false
}

fn condition_quote(_s: &Span, key: char, c: char, escape: bool, _index: usize) -> bool {
    if c == key && escape == false {
        return true;
    }

    false
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_distance(
    s: &Span,
    key: char,
    f: fn(&Span, char, char, bool, usize) -> bool,
) -> Option<usize> {
    let mut result: usize = 0;
    let mut escape = false;

    for (index, c) in s.as_bytes().iter().enumerate() {
        if f(s, key, *c as char, escape, index) == true {
            return Some(result);
        }

        if *c as char == '\\' {
            escape = match escape {
                true => false,
                false => true,
            }
        } else {
            escape = false;
        }

        result += 1;
    }

    None
}

fn parse_complex_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<Expr>, E>
where
    E: ParseError<Span<'a>>,
{
    let mut vector = vec![];

    let distance = match get_distance(&s, '}', condition_brace) {
        Some(result) => result,
        None => unreachable!(),
    };

    let (rest, string) = s.take_split(distance);

    let array = string.fragment().to_owned().split_ascii_whitespace();

    for (index, string) in array.into_iter().enumerate() {
        match index > 0 {
            true => {
                return Err(gen_nom_failure(s, ERROR_DOUBLE_QUOTE));
            }
            false => {
                let expr = Expr::IdentExpr(Identifier::new(string, Interval::new_as_u32(0, 0)));

                vector.push(expr);
            }
        }
    }

    Ok((rest, vector))
}

fn parse_simple_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let mut vector = vec![];

    match get_distance(&s, '\"', condition_quote) {
        Some(distance) => {
            let (rest, string) = s.take_split(distance);

            let mut string = string.to_owned();

            while !string.fragment().is_empty() {
                match (
                    get_distance(&string, '{', condition_brace),
                    get_distance(&string, '}', condition_brace),
                ) {
                    (Some(lhs_distance), Some(_)) => {
                        let (rest, value) = string.take_split(lhs_distance);

                        vector.push(Expr::LitExpr(PrimitiveString::get_literal(
                            value.fragment(),
                            Interval::new_as_u32(0, 0),
                        )));

                        let (rest, expression) =
                            delimited(tag(L2_BRACE), parse_complex_string, tag(R2_BRACE))(rest)?;

                        vector = [&vector[..], &expression[..]].concat();

                        string = rest;
                    }
                    (_, _) => {
                        let (rest, value) = string.take_split(string.fragment().len());

                        vector.push(Expr::LitExpr(PrimitiveString::get_literal(
                            value.fragment(),
                            Interval::new_as_u32(0, 0),
                        )));

                        string = rest;
                    }
                }
            }

            Ok((
                rest,
                Expr::ComplexLiteral(
                    vector,
                    RangeInterval::new(Interval::new_as_u32(0, 0), Interval::new_as_u32(0, 0)),
                ),
            ))
        }
        None => Err(gen_nom_failure(s, ERROR_DOUBLE_QUOTE)),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    delimited(tag(DOUBLE_QUOTE), parse_simple_string, tag(DOUBLE_QUOTE))(s)
}

////////////////////////////////////////////////////////////////////////////////
// TEST FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_comments::comment;
    use nom::sequence::preceded;

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
        let string = Span::new("  \"complex string {{ test }}\"  ");
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
    fn err_complex_string_no_left_bracket() {
        let string = Span::new("  \"complex string  }}\"  ");
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }
}
