use nom::error::{ErrorKind, ParseError};

#[derive(Clone, Debug, PartialEq)]
pub struct CustomError<I> {
    pub input: I,
    pub error: String,
}

impl<I> ParseError<I> for CustomError<I> {
    //TODO: update this in nom 6
    fn from_error_kind(input: I, _kind: ErrorKind) -> Self {
        CustomError {
            input,
            error: "".to_owned(),
        }
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }

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
