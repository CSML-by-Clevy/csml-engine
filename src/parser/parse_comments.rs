use crate::parser::tokens::*;
use crate::parser::tokens::Span;

use nom::*;

named!(pub comment_delimited<Span, Span>, preceded!(
    tag!(START_COMMENT),
    take_until_and_consume!(END_COMMENT)
));

named!(pub skip<Span, Vec<Span>>, do_parse!(
    vec: many0!(
        ws!(
            many0!(comment_delimited)
        )
    ) >>
    (vec.into_iter().flatten().collect())
));

#[macro_export]
macro_rules! comment (
    ($i:expr, $($args:tt)*) => (
        {
            use crate::parser::parse_comments::skip;
            sep!($i, skip, ws!($($args)*))
        }
    )
);
