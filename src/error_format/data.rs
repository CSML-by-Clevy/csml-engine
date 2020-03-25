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

    // trait ParseError
    // https://docs.rs/nom/5.0.1/src/nom/error.rs.html#13-40
}

#[repr(u32)]
pub enum ParserErrorType {
    StepDuplicateError = 0,
    AssignError = 1,
    AsError = 2,
    GotoStepError = 10,
    ImportError = 11,
    ImportStepError = 12,
    NoAscii = 13,
    AcceptError = 100,
    LeftBraceError = 110,
    RightBraceError = 111,
    LeftParenthesesError = 112,
    RightParenthesesError = 113,
    RightBracketError = 114,
    DoubleQuoteError = 120,
    DoubleBraceError = 130,
}

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub message: String,
    pub interval: Interval,
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
