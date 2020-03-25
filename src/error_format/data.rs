use crate::data::ast::Interval;
use nom::error::{ErrorKind, ParseError};

#[derive(Clone, Debug, PartialEq)]
pub struct CustomError<I> {
    pub input: I,
    pub error: String,
}

impl<I> ParseError<I> for CustomError<I> {
    //TODO: String?
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        CustomError {
            input,
            error: kind.description().to_owned(),
        }
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }

    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        other.input = input;
        other.error = ctx.to_owned();
        other
    }
}

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub interval: Interval,
    pub message: String,
}

impl ErrorInfo {
    pub fn format_error(&self) -> String {
        format!(
            "{} at line {}, column {}",
            self.message, self.interval.line, self.interval.column
        )
    }

    pub fn new(message: String, interval: Interval) -> Self {
        Self { message, interval }
    }
}
