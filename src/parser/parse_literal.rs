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
    i: map_res!(map_res!(digit, complete_byte_slice_str_from_utf8), complete_str_from_str) >>
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