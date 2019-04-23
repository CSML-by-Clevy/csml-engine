use crate::lexer::token::*;
use crate::lexer::tools::slice_to_utf8::complete_byte_slice_str_from_utf8;

use nom::*;
use nom::types::*;
use nom_locate::position;
use std::str::FromStr;

fn complete_str_from_str<F: FromStr>(c: CompleteStr) -> Result<F, F::Err> {
    FromStr::from_str(c.0)
}

named!(pub lex_integer<Span, Token>,
    do_parse!(
        position: position!() >>
        i: map_res!(map_res!(digit, complete_byte_slice_str_from_utf8), complete_str_from_str) >>
        (Token::IntLiteral(i, position))
    )
);
