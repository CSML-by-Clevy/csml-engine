use crate::lexer::token::*;

use nom::types::*;
use std::str;
use std::str::Utf8Error;

pub fn complete_byte_slice_str_from_utf8(c: Span) -> Result<CompleteStr, Utf8Error> {
    str::from_utf8(c.fragment.0).map(|s| CompleteStr(s))
}
