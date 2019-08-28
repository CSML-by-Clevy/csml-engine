use crate::parser::tokens::Span;
use crate::parser::tokens::*;

use nom::*;

// ####################

//TODO: check for errors 
#[macro_export]
macro_rules! take_until_and_consume_line (
  ($i:expr, $substr1:expr, $substr2:expr) => (
    {
      use nom::lib::std::result::Result::*;
      use nom::lib::std::option::Option::*;
      use nom::InputLength;
      use nom::FindSubstring;
      use nom::Slice;

      let input = $i;

      let res: IResult<_,_> = match (input.find_substring($substr1), input.find_substring($substr2)) {
        (Some(index), _) => {
          Ok(($i.slice(index+$substr1.input_len()..), $i.slice(0..index)))
        },
        (_, Some(index)) => {
          Ok(($i.slice(index+$substr2.input_len()..), $i.slice(0..index)))
        },
        (None, None) => {
          let index = $i.fragment.len();
          Ok(($i.slice(index..), $i.slice(0..index)))
        },
      };
      res
    }
  );
);

named!(pub comment_delimited<Span, Span>, preceded!(
    tag!(START_COMMENT),
    take_until_and_consume!(END_COMMENT)
));

named!(comment_single_line<Span, Span>, preceded!(
    tag!(INLINE_COMMENT),
    take_until_and_consume_line!("\n", "\r\n")
));

named!(pub all_comment<Span, Span>, alt!(
    comment_delimited   |
    comment_single_line
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
