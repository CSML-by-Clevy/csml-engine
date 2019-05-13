use crate::lexer::token::*;

use nom::*;
use nom_locate::position;

named!(comma_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(",") >> 
        (Token::Comma(position))
    )
);

named!(dot_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(".") >> 
        (Token::Dot(position))
    )
);

named!(semicolon_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(";") >> 
        (Token::SemiColon(position))
    )
);

named!(colon_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(":") >> 
        (Token::Colon(position))
    )
);

named!(lparen_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("(") >> 
        (Token::LParen(position))
    )
);

named!(rparen_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!(")") >> 
        (Token::RParen(position))
    )
);

named!(lbrace_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("{") >> 
        (Token::LBrace(position))
    )
);

named!(rbrace_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("}") >> 
        (Token::RBrace(position))
    )
);

named!(lbracket_punctuation<Span, Token>,
    do_parse!(
        position: position!() >>
        tag!("[") >> 
        (Token::LBracket(position))
    )
);

named!(rbracket_punctuation<Span, Token>, do_parse!(
        position: position!() >>
        tag!("]") >> 
        (Token::RBracket(position))
    )
);

// named!(space_punctuation<Span, Token>,
//     do_parse!(
//         alt!(
//             char!(' ') |
//             char!('\t')
//         )  >>
//         (Token::Space)
//     )
// );

// named!(newline_punctuation<Span, Token>,
//     do_parse!(
//        C >>
//         (Token::NewLine)
//     )
// );

named!(pub lex_punctuations<Span, Token>, alt!(
    comma_punctuation |
    dot_punctuation |
    semicolon_punctuation |
    colon_punctuation |
    lparen_punctuation |
    rparen_punctuation |
    lbrace_punctuation |
    rbrace_punctuation |
    lbracket_punctuation |
    rbracket_punctuation

    // space_punctuation |
    // newline_punctuation
));
