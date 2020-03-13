pub mod data;

use crate::data::{ast::Interval, tokens::Span};
use nom::{
    error::{ErrorKind, ParseError},
    *,
};

pub use data::{CustomError, ErrorInfo};

pub const ERROR_PARENTHESES: &'static str = "list elem type ( ... ) not found";
pub const ERROR_PARENTHESES_END: &'static str =
    "invalid argument expect one ',' between each argument or ')' to end the list";
pub const ERROR_NUMBER_AS_IDENT: &'static str = "int/float can't be used as identifier";
pub const ERROR_RESERVED: &'static str = "reserved keyword can't be used as identifier";
pub const ERROR_PARSING: &'static str =
    "invalid argument use one of [say, do, if, ..] keywords to start an action";
pub const ERROR_REMEMBER: &'static str =
    "remember must be assigning to a variable via '=': remember key = value";
pub const ERROR_USE: &'static str =
    "use must be assigning to a variable via 'as': use value as key";
pub const ERROR_BREAK: &'static str = "break can only be used inside a foreach";
pub const ERROR_HOLD: &'static str = "hold cannot be used inside a foreach";
pub const ERROR_LEFT_BRACE: &'static str = "expect '('";
pub const ERROR_RIGHT_BRACE: &'static str = "expect ')'";
pub const ERROR_RIGHT_BRACKET: &'static str = "expect ']'";
pub const ERROR_GOTO_STEP: &'static str = "missing step name after goto";
pub const ERROR_IMPORT_STEP: &'static str = "missing step name after import";
pub const ERROR_DOUBLE_QUOTE: &'static str = "expect '\"' to end string";

pub fn format_error<I>(
    interval: Interval,
    error_code: CustomError<I>,
    _code_error: &[u8],
) -> ErrorInfo {
    let message = error_code.error;
    ErrorInfo { interval, message }
}

pub fn gen_nom_error<'a, E>(span: Span<'a>, error: &'static str) -> Err<E>
where
    E: ParseError<Span<'a>>,
{
    Err::Error(E::add_context(
        span,
        error,
        E::from_error_kind(span, ErrorKind::Tag),
    ))
}

pub fn gen_nom_failure<'a, E>(span: Span<'a>, error: &'static str) -> Err<E>
where
    E: ParseError<Span<'a>>,
{
    Err::Failure(E::add_context(
        span,
        error,
        E::from_error_kind(span, ErrorKind::Tag),
    ))
}
