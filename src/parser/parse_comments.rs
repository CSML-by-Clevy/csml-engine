use crate::parser::tokens::Span;
use crate::parser::tokens::*;

use nom::*;

named!(pub comment_delimited<Span, Span>, preceded!(
    tag!(START_COMMENT),
    take_until_and_consume!(END_COMMENT)
));

// INLINE_COMMENT ->  //
// INLINE_COMMENT_HASH -> #
// regex!(r"^(?-u).*?(\r\n|\n|$)")
named!(
    comment_single_line<Span, Span>,
    preceded!(
        alt!(tag!(INLINE_COMMENT) | tag!(INLINE_COMMENT_HASH)),
        take_until_and_consume!("\n")
    )
);

named!(
    comment_single_line2<Span, Span>,
    preceded!(
        alt!(tag!(INLINE_COMMENT) | tag!(INLINE_COMMENT_HASH)),
        take_until_and_consume!("\r\n")
    )
);

named!(pub all_comment<Span, Span>, alt!(
    comment_delimited   |
    comment_single_line |
    comment_single_line2
));

named!(pub skip<Span, Vec<Span>>, do_parse!(
    vec: many0!(
        ws!(
            many0!(all_comment)
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
