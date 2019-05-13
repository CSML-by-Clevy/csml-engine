use crate::lexer::token::Span;
use nom::*;

named!(pub comment_delimited<Span, Span>,
    preceded!(
        tag!("/*"),
        take_until_and_consume!("*/")
    )
);

named!(pub whitespace<Span, Span>,
    is_a!(" \t\n\r")
);

named!(pub skip<Span, Vec<Span>>,
  many0!(
    alt!(
      comment_delimited |
      whitespace
    )
  )
);

#[macro_export]
macro_rules! comment (
  ($i:expr, $($args:tt)*) => (
    {
      use crate::lexer::lex_comments::skip;
      sep!($i, skip, $($args)*)
    }
  )
);
