use crate::parser::ast::Interval;

#[repr(u32)]
pub enum ParserErrorType {
    StepDuplicateError = 0,
    AssignError = 1,
    GotoStepError = 10,
    ImportError = 11,
    ImportStepError = 12,
    AcceptError = 100,
    LeftBraceError = 110,
    RightBraceError = 111,
    LeftParenthesesError = 112,
    RightParenthesesError = 113,
    RightBracketError = 114,
    DoubleQuoteError = 120,
    DoubleBraceError = 130,
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub message: String,
    pub interval: Interval,
}
