use crate::lexer::token::*;
use crate::lexer::tools::slice_to_utf8::complete_byte_slice_str_from_utf8;

use nom::*;
use nom::types::*;
use nom_locate::position;

macro_rules! check(
    ($input:expr, $submac:ident!( $($args:tt)* )) => (
        {
        use std::result::Result::*;
        use nom::{Err,ErrorKind};

        let mut failed = false;
        for &idx in $input.fragment.0 {
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

fn parse_reserved<'a>(c: CompleteStr, rest: Option<CompleteStr>, position: Span<'a>) -> Token<'a> {
    let mut string = c.0.to_owned();
    string.push_str(rest.unwrap_or(CompleteStr("")).0);
    match string.as_ref() {
        "if" => Token::If(position),
        "flow" => Token::Flow(position),
        "goto" => Token::Goto(position),
        "import" => Token::Import(position),
        
        "step" => Token::Step(position),
        "as" => Token::As(position),

        "remember" => Token::Remember(position),
        "retry" => Token::ReservedFunc(string, position),
        "say" => Token::ReservedFunc(string, position),

        "true" => Token::BoolLiteral(true, position),
        "false" => Token::BoolLiteral(false, position),
        _ => Token::Ident(string, position),
    }
}

named!(take_1_char<Span, Span>,
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
            c != '_' && !c.is_alphanumeric()
        },
        ErrorKind::Alpha,
    )
}

named!(pub lex_from_file<Span, Token>, do_parse!(
    position: position!() >>
    tag!("from")  >>
    tag!("file")  >>
    (Token::FromFile(position))
));

named!(pub lex_reserved_ident<Span, Token>,
    do_parse!(
        position: position!() >>
        c: map_res!(call!(take_1_char), complete_byte_slice_str_from_utf8) >>
        rest: opt!(complete!(map_res!(my_ascii, complete_byte_slice_str_from_utf8))) >>
        (parse_reserved(c, rest, position))
    )
);