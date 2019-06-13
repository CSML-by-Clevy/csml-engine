use crate::parser::tokens::*;
use crate::comment;

use nom_locate::position;
use nom::*;

named!(pub parse_ident<Span, String>, do_parse!(
    _position: position!() >>
    var: comment!(take_till1!(is_valid_char)) >>
    (String::from_utf8(var.fragment.to_vec()).expect("error at parsing [u8] to &str"))
));

pub fn is_valid_char(input: u8) -> bool {
    let var = input as char;
    input != b'_' && !var.is_alphanumeric()
}

