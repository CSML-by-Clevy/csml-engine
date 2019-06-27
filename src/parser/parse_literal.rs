use crate::parser::{tools::*, ast::*, tokens::*};
use crate::comment;

use nom::*;

named!(signed_digits<Span, Span>, recognize!(
    tuple!(
        opt!(alt!(tag!("+") | tag!("-"))),
        digit
    )
));

named!(pub parse_integer<Span, Expr>, do_parse!(
    i: map_res!(map_res!(signed_digits, complete_byte_slice_str_from_utf8), complete_str_from_str) >>
    (Expr::new_literal(Literal::IntLiteral(i)) )
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
    elem: map_res!(map_res!(floating_point, complete_byte_slice_str_from_utf8), complete_str_from_str) >>
    (Expr::new_literal(Literal::FloatLiteral(elem)))
));

named!(parse_boolean<Span, Expr>, do_parse!(
    boolean: alt!(
            do_parse!(
                tag!(TRUE) >>
                (Literal::BoolLiteral(true))

            ) |
            do_parse!(
                tag!(FALSE) >>
                (Literal::BoolLiteral(false))
            )
    ) >>
    (Expr::new_literal(boolean))
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
    (lit) //, span
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
            Ok(..) => {},
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn ok_float() {
        let string = Span::new(CompleteByteSlice(" -42.42 ".as_bytes()));
        match test_literal(string) {
            Ok(..) => {},
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn ok_bool() {
        let string = Span::new(CompleteByteSlice(" true ".as_bytes()));
        match test_literal(string) {
            Ok(..) => {},
            Err(e) => panic!("{:?}", e)
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
            Ok(ok)  => panic!("need to fail {:?}", ok),
            Err(..) => {}
        }
    }
}
