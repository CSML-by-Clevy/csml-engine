use crate::comment;
use crate::parser::{ast::*, tokens::*, tools::get_interval, parse_literal::get_int};

use nom::*;

named!(pub parse_ident<Span, SmartIdent>, do_parse!(
    position: get_interval >>
    var: comment!(take_till1!(is_valid_char)) >>
    index: opt!(
         delimited!(
            comment!(tag!(L_BRACKET)),
            get_int,
            comment!(tag!(R_BRACKET))
        )
    ) >>
    (forma_ident(
        String::from_utf8(var.fragment.to_vec()).expect("error parsing [u8] to &str at parse_ident"),
        index, 
        position
    ))
));

pub fn is_valid_char(input: u8) -> bool {
    let var = input as char;
    input != b'_' && !var.is_alphanumeric()
}

pub fn forma_ident(ident: String, index: Option<i64>, position: Interval) -> SmartIdent {
    Expr::new_ident(
        ident,
        position,
        index
    )
}