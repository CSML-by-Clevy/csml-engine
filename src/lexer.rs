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

// named!(double_quote_punctuation<Span, Token>, do_parse!(
//         position: position!() >>
//         tag!("\"") >> 
//         (Token::DoubleQuote(position))
//     )
// );

named!(space_punctuation<Span, Token>,
    do_parse!(
        alt!(
            char!(' ') |
            char!('\t')
        )  >>
        (Token::Space)
    )
);

named!(l2brace_punctuation<Span, Token>,
    do_parse!(
    position: position!() >>
    tag!("{{") >> (Token::L2Brace(position))
    )
);

named!(r2brace_punctuation<Span, Token>,
    do_parse!(
    position: position!() >>
    tag!("}}") >> (Token::R2Brace(position))
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

    l2brace_punctuation |
    r2brace_punctuation |
    // double_quote_punctuation |
    space_punctuation |
    newline_punctuation
));

// Strings #############################################

fn parse_string(input: Span) -> IResult<Span, Vec<u8> > {
    use std::result::Result::*;

    let (i1, c1) = try_parse!(input, take!(1));
    // println!("i1 {:?} c1 {:?}", i1, c1);
    match c1.fragment.as_bytes() {
        b"\"" => Ok((input, vec![])),
        c => parse_string(i1).map(|(slice, done)| {
                // println!("slice {:?}, done {:?}", slice, done);
                (slice, concat_slice_vec(c, done))
            }
        ),
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

named!(string<Span, String>,
    delimited!(
        tag!("\""),
        map_res!(parse_string, convert_vec_utf8),
        tag!("\"")
    )
);

named!(lex_string<Span, Token>,
    do_parse!(
        position: position!() >>
        s: string >>
        (Token::StringLiteral(s, position))
    )
);

// Strings 2
// //NOTE: ComplexString
named!(parse_2brace<Span, Token>, do_parse!(
    vec: delimited!(
        tag!("{{"), many0!(lex_token), tag!("}}")
    ) >>
    (Token::ComplexString(vec))
));

named!(get_position<Span, Span>, position!());

// //NOTE: ComplexString
named!(lex_complex_string<Span, Token>, do_parse!(
    vec: delimited!(
        tag!("\""), parse_string2, tag!("\"")
    ) >>
    (Token::ComplexString(vec))
));

//NOTE: ComplexString
fn parse_brace<'a>(input: Span<'a>, mut vec: Vec<Token<'a>>) -> IResult<Span<'a>, Vec<Token<'a> >> {

    if let Ok((rest, token)) = parse_2brace(input) {
        vec.push(token);
        if let Ok((rest2, mut vec2)) = parse_string2(rest) {
            vec.append(&mut vec2);
            println!("trtretretretertret");
            Ok((rest2, vec))
        } else {
            println!("hola 2 como estas 2");
            Ok((rest, vec))
        }
        
    } else {
        println!("asdasdasdas");
        // panic!("error need to check if brace are missing");
        Ok((input, vec))
    }
}

fn get_dist(input: &Span, val: &str) -> (usize, usize) {
    let dist1 = match input.find_substring(val) { 
        Some(len) => len,
        None      => 0,
    };
    let dist2 = match input.find_substring("\"") { 
        Some(len) => len,
        None      => 0,
    };
    (dist1, dist2)
}

//NOTE: ComplexString
fn parse_string2(input: Span) -> IResult<Span, Vec<Token>> {
    match get_dist(&input, "{{") {
        (d1, d2) if d1 < d2 => {
            let (rest, val) = input.take_split(d1);
            let (val, position) = get_position(val).unwrap();
            let mut vec = vec![];
            if val.input_len() > 0 {
                let string = convert_vec_utf8(val.fragment.as_bytes().to_vec()).unwrap();
                vec.push(Token::StringLiteral(string, position));
            }
            parse_brace(rest, vec)
        },
        (_, d2)             => {
            let (rest, val) = input.take_split(d2);
            let (val, position) = get_position(val).unwrap();
            let mut string: String = String::new();
            if val.input_len() > 0 {
                string = convert_vec_utf8(val.fragment.as_bytes().to_vec()).unwrap();
            }
            Ok((rest, vec![Token::StringLiteral(string, position)]))
        },
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
        // "execute"
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
    lex_complex_string |
    // lex_string |
    lex_reserved_ident |
    lex_illegal
));

named!(start_lex<Span, Vec<Token>>, many0!(lex_token));

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(slice: &[u8]) -> IResult<Span, Vec<Token>> {
        start_lex(Span::new(CompleteByteSlice(slice)))
            .map(|(slice, result)| (slice, [&result, &vec![Token::EOF][..]].concat()))
    }
}
