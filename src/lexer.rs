pub mod token;
pub mod tools;
pub mod lex_operators;
pub mod lex_illegal;
pub mod lex_reserved;
pub mod lex_punctuation;
pub mod lex_int;

use token::*;
use lex_operators::lex_operator;
use lex_punctuation::lex_punctuations;
use lex_illegal::lex_illegal;
use lex_reserved::lex_reserved_ident;
use lex_int::lex_integer;

use nom::*;
use nom::types::*;
use nom_locate::position;
use std::str;
use std::str::Utf8Error;

named!(lex_token<Span, Token>, alt_complete!(
    lex_operator |
    lex_punctuations |
    lex_integer |
    lex_string |
    lex_reserved_ident |
    lex_illegal
));

named!(start_lex<Span, Vec<Token>>, ws!(many0!(lex_token)));

fn concat_token<'a>(nested: &mut Vec<Token<'a>>, vec: &mut Vec<Token<'a>>) {
    for elem in nested.drain(..) {
        if let Token::ComplexString(mut vecval) = elem {
            concat_token(&mut vecval, vec);
        } else {
            vec.push(elem);
        }
    }
}

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

//  String #################################################################
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

//TODO: ERROR HANDLING
named!(pub lex_string<Span, Token>, do_parse!(
    vec: delimited!(
        tag!("\""), ws!(parse_string), tag!("\"")
    ) >>
    (Token::ComplexString( format_complex_string(vec) ))
));

//  String #################################################################