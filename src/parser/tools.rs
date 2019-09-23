use crate::comment;
use crate::parser::{ast::*, expressions_evaluation::*, tokens::*, ParserErrorType};

use nom::types::*;
use nom::*;
use nom_locate::position;
use std::str;
use std::str::{FromStr, Utf8Error};

named!(pub get_interval<Span, Interval>, do_parse!(
    position: position!() >>
    (Interval::new(position))
));

pub fn complete_byte_slice_str_from_utf8(c: Span) -> Result<CompleteStr, Utf8Error> {
    str::from_utf8(c.fragment.0).map(|s| CompleteStr(s))
}

pub fn complete_str_from_str<F: FromStr>(c: CompleteStr) -> Result<F, F::Err> {
    FromStr::from_str(c.0)
}

named!(pub parse_l_parentheses<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::LeftParenthesesError as u32),
    tag!(L_PAREN)
));

named!(pub parse_r_parentheses<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::RightParenthesesError as u32),
    tag!(R_PAREN)
));

named!(pub parse_strict_condition_group<Span, Expr>, delimited!(
    comment!(parse_l_parentheses),
    operator_precedence,
    comment!(parse_r_parentheses)
));

named!(pub parse_condition_group<Span, Expr>, delimited!(
    tag!(L_PAREN),
    operator_precedence,
    comment!(parse_r_parentheses)
));

named!(pub parse_r_bracket<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::RightBracketError as u32),
    tag!(R_BRACKET)
));

named!(pub parse_l_brace<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::LeftBraceError as u32),
    tag!(L_BRACE)
));

named!(pub parse_r_brace<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::RightBraceError as u32),
    tag!(R_BRACE)
));

named!(pub parse_import_step<Span, Span>, return_error!(
    nom::ErrorKind::Custom(ParserErrorType::ImportStepError as u32),
    tag!(STEP)
));
