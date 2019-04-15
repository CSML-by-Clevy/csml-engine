pub mod token;

use nom::types::*;
use nom::*;
use nom_locate::position;
use std::str;
use std::str::FromStr;
use std::str::Utf8Error;
use token::*;

// operators
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

named!(lex_operator<Span, Token>, alt!(
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

named!(space_punctuation<Span, Token>,
    do_parse!(
        alt!(
            char!(' ') |
            char!('\t')
        )  >>
        (Token::Space)
    )
);

named!(newline_punctuation<Span, Token>,
    do_parse!(
        char!('\n') >>
        (Token::NewLine)
    )
);

named!(lex_punctuations<Span, Token>, alt!(
    comma_punctuation |
    dot_punctuation |
    semicolon_punctuation |
    colon_punctuation |
    lparen_punctuation |
    rparen_punctuation |
    lbrace_punctuation |
    rbrace_punctuation |
    lbracket_punctuation |
    rbracket_punctuation |

    space_punctuation |
    newline_punctuation
));

// Strings #############################################

fn convert_vec_utf8(v: Vec<u8>) -> Result<String, Utf8Error> {
    let slice = v.as_slice();
    str::from_utf8(slice).map(|s| s.to_owned())
}

//TODO: ERROR HANDLING
named!(parse_2brace<Span, (Vec<Token>, Span) >, do_parse!(
    tag!("{{") >>
    vec: many_till!(ws!(lex_token) ,tag!("}}")) >>
    (vec)
));

named!(get_position<Span, Span>, position!());

fn not_stirng(token: &Token) -> bool {
    if let Token::StringLiteral(..) = token {
        false
    } else { 
        true 
    }
}

fn format_complex_string(mut vec: Vec<Token>) -> Vec<Token> {
    if vec.len() > 1 || (vec.len() == 1 && not_stirng(&vec[0]) ) {
        vec = [&[Token::L2Brace][..], &vec[..], &[Token::R2Brace][..]].concat();
    }
    vec
}

//TODO: ERROR HANDLING
named!(lex_string<Span, Token>, do_parse!(
    vec: delimited!(
        tag!("\""), ws!(parse_string), tag!("\"")
    ) >>
    (Token::ComplexString( format_complex_string(vec) ))
));

//TODO: ERROR HANDLING
fn parse_brace<'a>(input: Span<'a>, mut vec: Vec<Token<'a>>) -> IResult<Span<'a>, Vec<Token<'a> >> {
    match parse_2brace(input) {
        Ok((rest, (mut tokens, _))) => {
            vec.append(&mut tokens);
            if let Ok((rest2, mut vec2)) = parse_string(rest) {
                vec.append(&mut vec2);
                Ok((rest2, vec))
            } else {
                Ok((rest, vec))
            }
        },
        Err(_e) => {
            // if let Err::Error(res) = e {
            //     println!(" ERROR ;( {:?}", res)
            // }
            Ok((input, vec))
        }
    }
}

fn get_dist(input: &Span, val: &str) -> (Option<usize>, Option<usize>) {
    let dist1 = input.find_substring(val);
    let dist2 = input.find_substring("\"");
    (dist1, dist2)
}

//TODO: ERROR HANDLING
fn parse_string(input: Span) -> IResult<Span, Vec<Token>> {
    match get_dist(&input, "{{") {
        (Some(d1), Some(d2)) if d1 < d2 => {
            let (rest, val) = input.take_split(d1);
            let (val, position) = get_position(val).unwrap();

            let mut vec = vec![];
            if val.input_len() > 0 {
                let string = convert_vec_utf8(val.fragment.as_bytes().to_vec()).unwrap();
                vec.push(Token::StringLiteral(string, position));
            }
            match parse_brace(rest, vec) {
                Ok((res, value))  => {
                    Ok((res, value))
                },
                Err(_e) => {
                    Ok((rest, vec![]))
                },
            }
        },
        (_, Some(d2))             => {
            let (rest, val) = input.take_split(d2);
            let (val, position) = get_position(val).unwrap();
            let mut vec = vec![];

            if val.input_len() > 0 {
                vec.push(Token::StringLiteral(convert_vec_utf8(val.fragment.as_bytes().to_vec()).unwrap(), position));
            }
            Ok((rest, vec))
        },
        (_, _) => {
            // Return better Error
            Err(Err::Incomplete(Needed::Size(1)))
        }
    }
}

// Integers parsing ###########################################################
named!(lex_integer<Span, Token>,
    do_parse!(
        position: position!() >>
        i: map_res!(map_res!(digit, complete_byte_slice_str_from_utf8), complete_str_from_str) >>
        (Token::IntLiteral(i, position))
    )
);

fn complete_str_from_str<F: FromStr>(c: CompleteStr) -> Result<F, F::Err> {
    FromStr::from_str(c.0)
}

// Illegal tokens #############################################################
named!(lex_illegal<Span, Token>,
    do_parse!(
        position: position!() >>
        take!(1) >> 
        (Token::Illegal(position))
    )
);

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

// Reserved or ident
fn parse_reserved<'a>(c: CompleteStr, rest: Option<CompleteStr>, position: Span<'a>) -> Token<'a> {
    let mut string = c.0.to_owned();
    string.push_str(rest.unwrap_or(CompleteStr("")).0);
    match string.as_ref() {
        "if" => Token::If(position),
        "flow" => Token::Flow(position),
        "goto" => Token::Goto(position),
        "remember" => Token::Remember(position),

        "retry" => Token::ReservedFunc(string, position),
        "ask" => Token::ReservedFunc(string, position),
        "say" => Token::ReservedFunc(string, position),
        "import" => Token::ReservedFunc(string, position),

        "true" => Token::BoolLiteral(true, position),
        "false" => Token::BoolLiteral(false, position),
        _ => Token::Ident(string, position),
    }
}

fn complete_byte_slice_str_from_utf8(c: Span) -> Result<CompleteStr, Utf8Error> {
    str::from_utf8(c.fragment.0).map(|s| CompleteStr(s))
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
            c != '_' && !c.is_alphabetic()
        },
        ErrorKind::Alpha,
    )
}

named!(lex_reserved_ident<Span, Token>,
    do_parse!(
        position: position!() >>
        c: map_res!(call!(take_1_char), complete_byte_slice_str_from_utf8) >>
        rest: opt!(complete!(map_res!(my_ascii, complete_byte_slice_str_from_utf8))) >>
        (parse_reserved(c, rest, position))
    )
);

named!(lex_token<Span, Token>, alt_complete!(
    lex_operator |
    lex_punctuations |
    lex_integer |
    lex_string |
    lex_reserved_ident |
    lex_illegal
));

fn concat_token<'a>(nested: &mut Vec<Token<'a>>, vec: &mut Vec<Token<'a>>) {
    for elem in nested.drain(..) {
        if let Token::ComplexString(mut vecval) = elem {
            concat_token(&mut vecval, vec);
        } else {
            vec.push(elem);
        }
    }
}

named!(start_lex<Span, Vec<Token>>, ws!(many0!(lex_token)));

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(slice: &[u8]) -> IResult<Span, Vec<Token>> {
        start_lex(Span::new(CompleteByteSlice(slice)))
            .map(|(slice, result)| {
                        let mut newvec = vec![];
                        let mut vec = [&result, &vec![Token::EOF][..]].concat();
                        concat_token(&mut vec, &mut newvec);
                        (slice, newvec)
                    }
                )
    }
}