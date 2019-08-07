use crate::comment;
use crate::parser::{ast::*, tokens::*, tools::*};
use nom::*;

named!(signed_digits<Span, Span>, recognize!(
    tuple!(
        opt!(alt!(tag!("+") | tag!("-"))),
        digit
    )
));

named!(pub parse_integer<Span, Expr>, do_parse!(
    position: get_interval >>
    i: map_res!(map_res!(signed_digits, complete_byte_slice_str_from_utf8), complete_str_from_str) >>
    (Expr::new_literal(Literal::int(i), position))
));

named!(floating_point<Span, Span>, recognize!(
    tuple!(
        signed_digits,
        complete!(pair!(
            tag!("."),
            digit
        ))
    )
));

named!(pub parse_float<Span, Expr>, do_parse!(
    position: get_interval >>
    value: map_res!(map_res!(floating_point, complete_byte_slice_str_from_utf8), complete_str_from_str) >>
    (Expr::new_literal(Literal::float(value), position))
));

named!(parse_boolean<Span, Expr>, do_parse!(
    position: get_interval >>
    boolean: alt!(
            do_parse!(
                tag!(TRUE) >>
                (Literal::boolean(true))

            ) |
            do_parse!(
                tag!(FALSE) >>
                (Literal::boolean(false))
            )
    ) >>
    (Expr::new_literal(boolean, position))
));

named!(pub parse_literalexpr<Span, Expr>, do_parse!(
    // span: position!() >>
    lit: comment!(
        alt!(
            parse_float     |
            parse_integer   |
            parse_boolean
        )
    ) >>
    (lit)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::*;

    named!(pub test_literal<Span, Expr>, exact!(parse_literalexpr));

    #[test]
    fn ok_int() {
        let string = Span::new(CompleteByteSlice(" +42 ".as_bytes()));
        match test_literal(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_float() {
        let string = Span::new(CompleteByteSlice(" -42.42 ".as_bytes()));
        match test_literal(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_bool() {
        let string = Span::new(CompleteByteSlice(" true ".as_bytes()));
        match test_literal(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_sign() {
        let string = Span::new(CompleteByteSlice(" +++++4 ".as_bytes()));
        match test_literal(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_float1() {
        let string = Span::new(CompleteByteSlice(" 2.2.2 ".as_bytes()));
        match test_literal(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_float2() {
        let string = Span::new(CompleteByteSlice(" 3,2 ".as_bytes()));
        match test_literal(string) {
            Ok(ok) => panic!("need to fail {:?}", ok),
            Err(..) => {}
        }
    }
}
