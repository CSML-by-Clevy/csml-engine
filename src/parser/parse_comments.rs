use crate::parser::tokens::Span;
use crate::parser::tokens::*;

use nom::*;

// ####################

//TODO: check for errors 
#[macro_export]
macro_rules! take_until_and_consume_line (
  ($i:expr, $substr:expr) => (
    {
      use nom::lib::std::result::Result::*;
      use nom::lib::std::option::Option::*;
      use nom::InputLength;
      use nom::FindSubstring;
      use nom::Slice;

      let input = $i;

      let res: IResult<_,_> = match input.find_substring($substr) {
        None => {
          let index = $i.fragment.len();
          Ok(($i.slice(index..), $i.slice(0..index)))
        },
        Some(index) => {
          Ok(($i.slice(index+$substr.input_len()..), $i.slice(0..index)))
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
    take_until_and_consume_line!("\n")
));

named!(comment_single_line2<Span, Span>, preceded!(
    tag!(INLINE_COMMENT),
    take_until_and_consume_line!("\r\n")
));

named!(single_line<Span, Span>, alt!(
    comment_single_line |
    comment_single_line2
));


named!(comment_single<Span, Span>, preceded!(
    tag!(INLINE_COMMENT),
    not_line_ending
));

pub fn test(input: Span) -> bool {
    
    match not_line_ending(input) {
        Ok(var) => {
            println!("OK {:?}", var);
            true
        },
        Err(var)  => {
            println!("ERR {:?}", var);
            false
        }
    }
}

// ####################

named!(pub all_comment<Span, Span>, alt!(
    comment_delimited   |
    single_line
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
