pub mod ast;

use crate::lexer::token::*;
use nom::*;
use ast::*;


macro_rules! tag_token (
    ($i: expr, $tag: expr) => (
        {
            use std::result::Result::*;
            use nom::{Err,ErrorKind};

            let (i1, t1) = try_parse!($i, take!(1));

            if t1.tok.is_empty() {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                if t1.tok[0] == $tag {
                    Ok((i1, t1))
                } else {
                    Err(Err::Error(error_position!($i, ErrorKind::Count)))
                }
            }
        }
    );
);

// named!(parse_stmt<Token, Vec<i32> >,
//     vec![]
// );

named!(parse_program<Tokens, Vec<i32> >,
    do_parse!(
        // prog: many0!(parse_stmt) >>
        tag_token!(Token::EOF) >>
        // (prog)
        (vec![1,2,3,4])
    )
);

pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: Tokens) -> IResult<Tokens, Vec<i32>> {
        parse_program(tokens)
    }
}
