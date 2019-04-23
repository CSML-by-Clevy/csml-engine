use crate::lexer::token::*;

use nom::*;
use nom_locate::position;

named!(equal_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("==") >> 
        (Token::Equal(position))
    )
);

named!(or_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("||") >> 
        (Token::Or(position))
    )
);

named!(and_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("&&")  >> 
        (Token::And(position))
    )
);

named!(assign_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("=")   >> 
        (Token::Assign(position))
    )
);

named!(greaterthanequal_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(">=")  >> 
        (Token::GreaterThanEqual(position))
        )
);

named!(lessthanequal_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("<=")  >> 
        (Token::LessThanEqual(position))
    )
);

named!(greaterthan_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(">")   >> 
        (Token::GreaterThan(position))
    )
);

named!(lessthan_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("<")   >> 
        (Token::LessThan(position))
    )
);

named!(pub lex_operator<Span, Token>, alt!(
    equal_operator |
    assign_operator |
    or_operator |
    and_operator |
    greaterthanequal_operator |
    lessthanequal_operator |
    greaterthan_operator |
    lessthan_operator
    )
);