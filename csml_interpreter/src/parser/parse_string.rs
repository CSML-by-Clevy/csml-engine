use crate::data::primitive::string::PrimitiveString;
use crate::data::{ast::*, position::Position, tokens::*, Data, Literal, MessageData, MSG};
use crate::error_format::{gen_nom_failure, CustomError, *};
use crate::interpreter::variable_handler::expr_to_literal;
use crate::parser::operator::parse_operator;
use crate::parser::parse_comments::comment;
use crate::parser::tools::{get_distance_brace, get_interval, get_range_interval, parse_error};
use nom::{
    bytes::complete::tag,
    combinator::cut,
    error::ParseError,
    sequence::{delimited, preceded},
    *,
};
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// TOOL FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn add_to_vector<'a, E>(
    s: Span<'a>,
    length: usize,
    expr_vector: &mut Vec<Expr>,
    interval_vector: &mut Vec<Interval>,
) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, value) = s.take_split(length);
    let (value, interval) = get_interval(value)?;

    let mut string = String::new();
    let mut escape = false;

    for c in value.fragment().chars() {
        if c != '\\' || escape {
            if escape {
                match c {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '\'' => string.push('\''),
                    '\"' => string.push('\"'),
                    '\\' => string.push('\\'),
                    c => string.push(c),
                }

                escape = false;
            } else {
                string.push(c);
            }
        } else {
            escape = true;
        }
    }

    if !value.fragment().is_empty() {
        expr_vector.push(Expr::LitExpr {
            literal: PrimitiveString::get_literal(&string, interval),
            in_in_substring: false,
        });

        interval_vector.push(interval);
    }

    Ok((rest, value))
}

fn parse_close_bracket<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    match preceded(comment, tag("}}"))(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((s, _err))) | Err(Err::Failure((s, _err))) => {
            Err(gen_nom_failure(s, ERROR_WRONG_ARGUMENT_EXPANDABLE_STRING))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

fn get_distance_quote<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Option<usize>, E>
where
    E: ParseError<Span<'a>>,
{
    let mut escape = false;
    let mut index = 0;

    for (i_bytes, c) in s.as_bytes().iter().enumerate() {
        if *c as char == '\"' && !escape {
            return Ok((s, Some(i_bytes)));
        }

        if *c as char == '\\' {
            escape = match escape {
                true => false,
                false => true,
            }
        } else {
            escape = false;
        }
        index = i_bytes;
    }

    let (s, _) = nom::bytes::complete::take(index)(s)?;
    Ok((s, None))
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_complex_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (rest, expr) = match parse_operator(s) {
        Ok((rest, val)) => (rest, val),
        Err(Err::Error(_e)) => {
            let (_, interval) = get_interval(s)?;
            let expr = Expr::LitExpr {
                literal: PrimitiveString::get_literal("", interval),
                in_in_substring: true,
            };

            (s, expr)
        }
        Err(Err::Failure(e)) => return Err(Err::Failure(e)),
        Err(Err::Incomplete(needed)) => {
            return Err(Err::Incomplete(needed));
        }
    };

    Ok((rest, expr))
}

fn do_parse_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    match get_distance_quote(s)? {
        (_, Some(distance)) => {
            let (rest, string) = s.take_split(distance);

            let mut vector = vec![];
            let mut interval = vec![];
            let mut string = string.to_owned();

            while !string.fragment().is_empty() {
                match (
                    get_distance_brace(&string, '{'),
                    get_distance_brace(&string, '}'),
                ) {
                    (Some(lhs_distance), Some(rhs_distance)) if lhs_distance < rhs_distance => {
                        let (rest, _) =
                            add_to_vector(string, lhs_distance, &mut vector, &mut interval)?;
                        let (rest, expression) =
                            delimited(tag("{{"), parse_complex_string, parse_close_bracket)(rest)?;
                        vector.push(expression);

                        string = rest;
                    }
                    (Some(_), None) => {
                        return Err(gen_nom_failure(s, ERROR_DOUBLE_CLOSE_BRACE));
                    }
                    (None, Some(_)) => {
                        return Err(gen_nom_failure(s, ERROR_DOUBLE_OPEN_BRACE));
                    }
                    (_, _) => {
                        let (rest, _) = add_to_vector(
                            string,
                            string.fragment().len(),
                            &mut vector,
                            &mut interval,
                        )?;

                        string = rest;
                    }
                }
            }

            let interval = get_range_interval(&interval);

            Ok((rest, Expr::ComplexLiteral(vector, interval)))
        }
        (s, None) => Err(gen_nom_failure(s, ERROR_DOUBLE_QUOTE)),
    }
}

fn do_parse_expand_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    match s.find_substring("\\\"") {
        Some(distance) => {
            let (rest, string) = s.take_split(distance);
            Ok((
                rest,
                Expr::LitExpr {
                    literal: PrimitiveString::get_literal(
                        string.fragment(),
                        Interval::new_as_u32(0, 0, 0, None, None),
                    ),
                    in_in_substring: true,
                },
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
    let (start, _) = get_interval(s)?;

    match (
        tag(DOUBLE_QUOTE)(s) as IResult<Span<'a>, Span<'a>, E>,
        tag(BACKSLASH_DOUBLE_QUOTE)(s) as IResult<Span<'a>, Span<'a>, E>,
    ) {
        (Ok(_), ..) => parse_error(
            start,
            s,
            delimited(tag(DOUBLE_QUOTE), do_parse_string, cut(tag(DOUBLE_QUOTE))),
        ),
        (.., Ok(_)) => parse_error(
            start,
            s,
            delimited(
                tag(BACKSLASH_DOUBLE_QUOTE),
                do_parse_expand_string,
                tag(BACKSLASH_DOUBLE_QUOTE),
            ),
        ),
        (Err(err), ..) => Err(err),
    }
}

pub fn interpolate_string(
    string: &str,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let string_formatted = format!("{:?}", string);
    let span = Span::new(&string_formatted);

    match parse_string::<CustomError<Span>>(span) {
        Ok((span, expr)) => {
            if !span.fragment().is_empty() {
                Err(gen_error_info(
                    Position::new(
                        Interval::new_as_u32(
                            span.location_line(),
                            span.get_column() as u32,
                            span.location_offset(),
                            None,
                            None,
                        ),
                        &data.context.flow,
                    ),
                    ERROR_PARSING.to_owned(),
                ))
            } else {
                expr_to_literal(&expr, false, None, data, msg_data, sender)
            }
        }
        Err(e) => match e {
            Err::Error(err) | Err::Failure(err) => Err(gen_error_info(
                Position::new(Interval::new_as_u32(
                    err.input.location_line(),
                    err.input.get_column() as u32,
                    span.location_offset(),
                    None,
                    None,
                ),
                &data.context.flow,
            ),
                err.error,
            )),
            Err::Incomplete(_err) => unimplemented!(),
        },
    }
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

    ////////////////////////////////////////////////////////////////////////////
    /// SIMPLE STRINGS
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn ok_simple() {
        let string = "\"Hello\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_simple_reverse() {
        let string = "\"}} {{\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_simple_escape() {
        let string = "\"\\\"Hello\\\"\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_simple_multiple_arguments() {
        let string = "\"Hello World\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_simple_escape_multiple_arguments() {
        let string = "\"Hello \\\"World\\\"\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_simple_escape_quotes() {
        let string = "\"\\\"\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_simple_escape_open_brace() {
        let string = "\"\\{{\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_simple_escape_close_brace() {
        let string = "\"\\}}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_simple() {
        let string = "\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => panic!("need to fail"),
            Err(_) => {}
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// EXPAND STRINGS
    //////////////////////////////////////////////////////////////////////////

    #[test]
    fn ok_expand_empty_0() {
        let string = "\"{{ }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_empty_1() {
        let string = "\"{{}}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_integer() {
        let string = "\"{{ 42 }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_escape_string() {
        let string = "\"{{ \\\"Hello\\\" }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_escape_empty_string() {
        let string = "\"{{ \\\"\\\" }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_ident() {
        let string = "\"{{ Hello }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_array() {
        let string = "\"{{ [\\\"Hello\\\"] }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_empty_array() {
        let string = "\"{{ [] }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_object() {
        let string = "\"{{ {\\\"Foo\\\":\\\"Bar\\\"} }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_empty_object() {
        let string = "\"{{ {} }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_function_0() {
        let string = "\"{{ f() }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_function_1() {
        let string = "\"{{ f(\\\"hello\\\") }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_function_2() {
        let string = "\"{{ f(\\\"hello\\\", f(hello)) }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_as() {
        let string = "\"{{ [\\\"{{ Hello }}\\\"] as array }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_expand_expand_0() {
        let string = "\"{{ \\\"{{ Hello }}\\\" }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_expand_open() {
        let string = "\"{{ Hello\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => panic!("need to fail"),
            Err(_) => {}
        }
    }

    #[test]
    fn err_expand_close() {
        let string = "\"Hello }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => panic!("need to fail"),
            Err(_) => {}
        }
    }

    #[test]
    fn err_expand_multiple_arguments() {
        let string = "\"{{ Hello World }}\"";
        let span = Span::new(string);

        match test_string(span) {
            Ok(..) => panic!("need to fail"),
            Err(_) => {}
        }
    }
}
