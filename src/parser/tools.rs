use crate::parser::{ast::*, tokens::*};
use crate::comment;

use nom::*;
use nom::types::*;
use std::str;
use std::str::{Utf8Error, FromStr};

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

pub fn complete_byte_slice_str_from_utf8(c: Span) -> Result<CompleteStr, Utf8Error> {
    str::from_utf8(c.fragment.0).map(|s| CompleteStr(s))
}

fn complete_str_from_str<F: FromStr>(c: CompleteStr) -> Result<F, F::Err> {
    FromStr::from_str(c.0)
}

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