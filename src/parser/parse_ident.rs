use crate::comment;
use crate::parser::ast::*;
use crate::parser::tokens::*;

use nom::*;
use nom_locate::position;

named!(pub parse_ident<Span, SmartIdent>, do_parse!(
    position: position!() >>
    var: comment!(take_till1!(is_valid_char)) >>
    (Expr::new_ident(
        String::from_utf8(var.fragment.to_vec()).expect("error at parsing [u8] to &str"),
        position
    ))
));

pub fn is_valid_char(input: u8) -> bool {
    let var = input as char;
    input != b'_' && !var.is_alphanumeric()
}
