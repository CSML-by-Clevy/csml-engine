pub mod token;

use nom::types::*;
use nom::*;
use std::str;
use std::str::FromStr;
use std::str::Utf8Error;
use token::Token;

// operators
named!(equal_operator<CompleteByteSlice, Token>,
    do_parse!(tag!("==") >> (Token::Equal))
);

named!(or_operator<CompleteByteSlice, Token>,
    do_parse!(tag!("||") >> (Token::Or))
);

named!(and_operator<CompleteByteSlice, Token>,
    do_parse!(tag!("&&") >> (Token::And))
);

named!(assign_operator<CompleteByteSlice, Token>,
    do_parse!(tag!("=") >> (Token::Assign))
);

named!(greaterthanequal_operator<CompleteByteSlice, Token>,
    do_parse!(tag!(">=") >> (Token::GreaterThanEqual))
);

named!(lessthanequal_operator<CompleteByteSlice, Token>,
    do_parse!(tag!("<=") >> (Token::LessThanEqual))
);

named!(greaterthan_operator<CompleteByteSlice, Token>,
    do_parse!(tag!(">") >> (Token::GreaterThan))
);

named!(lessthan_operator<CompleteByteSlice, Token>,
    do_parse!(tag!("<") >> (Token::LessThan))
);

named!(lex_operator<CompleteByteSlice, Token>, alt!(
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

// punctuations
named!(comma_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!(",") >> (Token::Comma))
);

named!(dot_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!(".") >> (Token::Dot))
);

named!(semicolon_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!(";") >> (Token::SemiColon))
);

named!(colon_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!(":") >> (Token::Colon))
);

named!(lparen_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!("(") >> (Token::LParen))
);

named!(rparen_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!(")") >> (Token::RParen))
);

named!(l2brace_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!("{{") >> (Token::L2Brace))
);

named!(r2brace_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!("}}") >> (Token::R2Brace))
);

named!(lbrace_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!("{") >> (Token::LBrace))
);

named!(rbrace_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!("}") >> (Token::RBrace))
);

named!(lbracket_punctuation<CompleteByteSlice, Token>,
    do_parse!(tag!("[") >> (Token::LBracket))
);

named!(rbracket_punctuation<CompleteByteSlice, Token>, do_parse!(
        tag!("]") >> (Token::RBracket)
    )
);

// named!(new_line<CompleteByteSlice, Token>, do_parse!(
//         line_ending >> (Token::NewL)
//     )
// );

named!(lex_punctuations<CompleteByteSlice, Token>, alt!(
    comma_punctuation |
    dot_punctuation |
    semicolon_punctuation |
    colon_punctuation |
    lparen_punctuation |
    rparen_punctuation |
    l2brace_punctuation |
    r2brace_punctuation |
    lbrace_punctuation |
    rbrace_punctuation |
    lbracket_punctuation |
    rbracket_punctuation
    // new_line
));

// Strings
fn parse_string(input: CompleteByteSlice) -> IResult<CompleteByteSlice, Vec<u8>> {
    use std::result::Result::*;

    let (i1, c1) = try_parse!(input, take!(1));
    match c1.as_bytes() {
        b"\"" => Ok((input, vec![])),
        c => parse_string(i1).map(|(slice, done)| (slice, concat_slice_vec(c, done))),
    }
}

fn concat_slice_vec(c: &[u8], done: Vec<u8>) -> Vec<u8> {
    let mut new_vec = c.to_vec();
    new_vec.extend(&done);
    new_vec
}

fn convert_vec_utf8(v: Vec<u8>) -> Result<String, Utf8Error> {
    let slice = v.as_slice();
    str::from_utf8(slice).map(|s| s.to_owned())
}

named!(string<CompleteByteSlice, String>,
    delimited!(
        tag!("\""),
        map_res!(parse_string, convert_vec_utf8),
        tag!("\"")
    )
);

// named!(lex_label<CompleteByteSlice, Token>,
//     do_parse!(
//         s: string >>
//         tag!(":") >>
//         (Token::Label(s))
//     )
// );

named!(lex_string<CompleteByteSlice, Token>,
    do_parse!(
        s: string >>
        (Token::StringLiteral(s))
    )
);

// Integers parsing
named!(lex_integer<CompleteByteSlice, Token>,
    do_parse!(
        i: map_res!(map_res!(digit, complete_byte_slice_str_from_utf8), complete_str_from_str) >>
        (Token::IntLiteral(i))
    )
);

fn complete_str_from_str<F: FromStr>(c: CompleteStr) -> Result<F, F::Err> {
    FromStr::from_str(c.0)
}

// Illegal tokens
named!(lex_illegal<CompleteByteSlice, Token>,
    do_parse!(take!(1) >> (Token::Illegal))
);

macro_rules! check(
    ($input:expr, $submac:ident!( $($args:tt)* )) => (
        {
        use std::result::Result::*;
        use nom::{Err,ErrorKind};

        let mut failed = false;
        for &idx in $input.0 {
            if !$submac!(idx, $($args)*) {
                failed = true;
                break;
            }
        }
        if failed {
            let e: ErrorKind<u32> = ErrorKind::Tag;
            Err(Err::Error(error_position!($input, e)))
        } else {
            Ok((&b""[..], $input))
        }
        }
    );
    ($input:expr, $f:expr) => (
        check!($input, call!($f));
    );
);

// Reserved or ident
fn parse_reserved(c: CompleteStr, rest: Option<CompleteStr>) -> Token {
    let mut string = c.0.to_owned();
    string.push_str(rest.unwrap_or(CompleteStr("")).0);
    match string.as_ref() {
        "if" => Token::If,
        "flow" => Token::Flow,
        "goto" => Token::Goto,
        "remember" => Token::Remember,

        "retry" => Token::ReservedFunc(string),
        "ask" => Token::ReservedFunc(string),
        "say" => Token::ReservedFunc(string),
        "import" => Token::ReservedFunc(string),

        "True" => Token::BoolLiteral(true),
        "False" => Token::BoolLiteral(false),
        // "execute"
        _ => Token::Ident(string),
    }
}

fn complete_byte_slice_str_from_utf8(c: CompleteByteSlice) -> Result<CompleteStr, Utf8Error> {
    str::from_utf8(c.0).map(|s| CompleteStr(s))
}

named!(take_1_char<CompleteByteSlice, CompleteByteSlice>,
    flat_map!(take!(1), check!(is_alphabetic))
);

pub fn my_ascii<T>(input: T) -> IResult<T, T, u32>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1(
        |item| {
            let c = item.as_char();
            c != '_' && !c.is_alphabetic()
        },
        ErrorKind::Alpha,
    )
}

named!(lex_reserved_ident<CompleteByteSlice, Token>,
    do_parse!(
        c: map_res!(call!(take_1_char), complete_byte_slice_str_from_utf8) >>
        rest: opt!(complete!(map_res!(my_ascii, complete_byte_slice_str_from_utf8))) >>
        (parse_reserved(c, rest))
    )
);

named!(lex_token<CompleteByteSlice, Token>, alt_complete!(
    lex_operator |
    lex_punctuations |
    lex_integer |
    lex_string |
    lex_reserved_ident |
    lex_illegal
));

named!(start_lex<CompleteByteSlice, Vec<Token>>, ws!(many0!(lex_token)));

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(slice: &[u8]) -> IResult<CompleteByteSlice, Vec<Token>> {
        start_lex(CompleteByteSlice(slice))
            .map(|(slice, result)| (slice, [&result, &vec![Token::EOF][..]].concat()))
    }
}
