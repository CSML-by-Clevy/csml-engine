use nom::error::{ErrorKind, ParseError, ContextError, FromExternalError};

#[derive(Clone, Debug, PartialEq)]
pub struct CustomError<I> {
    pub input: I,
    pub end: Option<I>,
    pub error: String,
}

impl<I: std::fmt::Display> ParseError<I> for CustomError<I> {
    //TODO: update this in nom 6
    fn from_error_kind(input: I, _kind: ErrorKind) -> Self {
        CustomError {
            input,
            end: None,
            error: "".to_owned(),
        }
    }

    fn append(input: I, _kind: ErrorKind, other: Self) -> Self {
        Self {
            input: input,
            end: Some(other.input),
            error: other.error,
        }
    }
}

impl<I: std::fmt::Display> ContextError<I> for CustomError<I> {

    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        match other.error {
            error if "" == error => {
                other.input = input;
                other.error = ctx.to_owned();
                other
            }
            _ => other,
        }
    }
}

impl<I, E> FromExternalError<I, E> for CustomError<I> {
    /// Create a new error from an input position and an external error
    fn from_external_error(input: I, _kind: ErrorKind, _e: E) -> Self {
        CustomError {
            input,
            end: None,
            error: "".to_owned(),
        }
    }
  }