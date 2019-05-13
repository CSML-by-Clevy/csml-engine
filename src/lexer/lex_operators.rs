use crate::lexer::token::*;
use nom::*;
use nom_locate::position;

named!(equal_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(EQUAL) >> 
        (Token::Equal(position))
    )
);

named!(or_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(OR) >> 
        (Token::Or(position))
    )
);

named!(and_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(AND)  >> 
        (Token::And(position))
    )
);

named!(assign_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(ASSIGN)   >> 
        (Token::Assign(position))
    )
);

named!(greaterthanequal_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(GREATER_THAN_EQUAL)  >> 
        (Token::GreaterThanEqual(position))
        )
);

named!(lessthanequal_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(LESS_THAN_EQUAL)  >> 
        (Token::LessThanEqual(position))
    )
);

named!(greaterthan_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(GREATER_THAN)   >> 
        (Token::GreaterThan(position))
    )
);

named!(lessthan_operator<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(LESS_THAN)   >> 
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