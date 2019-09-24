use crate::parser::{
    ast::*,
    literal::Literal,
    parse_var_types::parse_var_expr,
    tokens::*,
    tools::get_interval,
    ParserErrorType,
};
use nom::*;
use nom::{Err, ErrorKind as NomError};
use nom_locate::position;
use std::str;

named!(parse_2brace<Span, (Vec<Expr>, Span)>, do_parse!(
    tag!(L2_BRACE) >>
    vec: many_till!(parse_var_expr, tag!(R2_BRACE)) >>
    (vec)
));

fn parse_brace<'a>(input: Span<'a>, mut vec: Vec<Expr>) -> IResult<Span<'a>, Expr> {
    match parse_2brace(input) {
        Ok((rest, (mut exprs, _))) => {
            vec.append(&mut exprs);

            match parse_complex_string(rest) {
                Ok((rest2, Expr::ComplexLiteral(mut vec2, range))) => {
                    vec.append(&mut vec2);
                    // TODO: BAD RANGE this is only for test
                    Ok((rest2, Expr::ComplexLiteral(vec, range)))
                }
                Ok((rest2, expr)) => {
                    if vec.is_empty() {
                        Ok((rest2, expr))
                    } else {
                        vec.push(expr);
                        let (rest2, p) = get_interval(rest2)?;
                        Ok((
                            rest2,
                            Expr::ComplexLiteral(
                                vec,
                                RangeInterval {
                                    start: p.clone(),
                                    end: p,
                                },
                            ),
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

fn parse_complex_string(input: Span) -> IResult<Span, Expr> {
    match get_distance(&input, L2_BRACE) {
        (Some(distance_to_l2brace), Some(distance_double_quote))
            if distance_to_l2brace < distance_double_quote =>
        {
            let (rest, val) = input.take_split(distance_to_l2brace);
            let (val, position) = get_interval(val)?;
            let mut vec = vec![];

            if val.input_len() > 0 {
                let value = String::from_utf8(val.fragment.to_vec())
                    .expect("error at parsing [u8] to &str");
                vec.push(Expr::LitExpr(Literal::string(value, position)));
            }
            match parse_brace(rest, vec) {
                Ok((rest, vec)) => Ok((rest, vec)),
                Err(Err::Failure(e)) => Err(Err::Failure(e)),
                _ => Err(Err::Failure(Context::Code(
                    input,
                    NomError::Custom(ParserErrorType::DoubleBraceError as u32),
                ))),
            }
        }
        (_, Some(distance_double_quote)) => {
            let (rest, val) = input.take_split(distance_double_quote);
            let (val, position) = get_interval(val)?;

            if val.input_len() > 0 {
                let value = String::from_utf8(val.fragment.to_vec())
                    .expect("error at parsing [u8] to &str");
                return Ok((rest, Expr::LitExpr(Literal::string(value, position))));
            }

            let (_val, position2) = get_interval(val)?;
            Ok((
                rest,
                Expr::ComplexLiteral(
                    vec![],
                    RangeInterval {
                        start: position,
                        end: position2,
                    },
                ),
            ))
        }
        (_, None) => Err(Err::Failure(Context::Code(
            input,
            NomError::Custom(ParserErrorType::DoubleQuoteError as u32),
        ))),
    }
}

named!(pub parse_string<Span, Expr>, do_parse!(
    _position: position!() >>
    expr: delimited!(
        tag!(DOUBLE_QUOTE), parse_complex_string, tag!(DOUBLE_QUOTE)
    ) >>

    (expr)
));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comment;
    use nom::types::*;

    named!(pub test_string<Span, Expr>, comment!(parse_string));

    #[test]
    fn ok_normal_string() {
        let string = Span::new(CompleteByteSlice("\"normal string\"".as_bytes()));
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_comment_string() {
        let string = Span::new(CompleteByteSlice(
            "    \"normal string\"    /* test */".as_bytes(),
        ));
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_normal_string_no_right_quote() {
        let string = Span::new(CompleteByteSlice(" \"normal string ".as_bytes()));
        match test_string(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_normal_string_no_left_quote() {
        let string = Span::new(CompleteByteSlice(" normal string\" ".as_bytes()));
        match test_string(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn ok_complex_string() {
        let string = Span::new(CompleteByteSlice(
            "  \"complex string {{ \"test\" }}\"  ".as_bytes(),
        ));
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_complex_complex_string() {
        let string = Span::new(CompleteByteSlice(
            "  \"complex string {{ \"var {{ \"test\" }}\" }}\"  ".as_bytes(),
        ));
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_complex_string_no_right_braket() {
        let string = Span::new(CompleteByteSlice("  \"complex string {{ \"  ".as_bytes()));
        match test_string(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_complex_string_no_left_braket() {
        let string = Span::new(CompleteByteSlice("  \"complex string  }}\"  ".as_bytes()));
        match test_string(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }
}
