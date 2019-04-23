use crate::lexer::token::*;

use nom::*;
use nom_locate::position;

named!(pub lex_illegal<Span, Token>,
    do_parse!(
        position: position!() >>
        take!(1) >> 
        (Token::Illegal(position))
    )
);