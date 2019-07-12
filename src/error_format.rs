pub mod data;

use crate::parser::ast::Interval;
use data::*;
use nom::ErrorKind;

pub fn get_error_message(error_code: ErrorKind) -> String {
    match error_code {
        ErrorKind::Custom(val) if val == ParserErrorType::StepDuplicateError as u32 => {
            "ERROR: Step name already exists".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::AssignError as u32 => {
            "ERROR: Missing = after remember statement".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::GotoStepError as u32 => {
            "ERROR: Missing label name after goto".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::AcceptError as u32 => {
            "ERROR: Flow argument expect Accept identifier".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::LeftBraceError as u32 => {
            "ERROR: Missing start of block { ".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::RightBraceError as u32 => {
            "ERROR: Agruments inside brace bad format or brace missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::LeftParenthesesError as u32 => {
            "ERROR: ( mabe missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::RightParenthesesError as u32 => {
            "ERROR: Agruments inside parentheses bad format or ) missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::RightBracketError as u32 => {
            "ERROR: Agruments inside parentheses bad format or ] missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::DoubleQuoteError as u32 => {
            "ERROR: \" mabe missing".to_string()
        }
        ErrorKind::Custom(val) if val == ParserErrorType::DoubleBraceError as u32 => {
            "ERROR: }} mabe missing".to_string()
        }
        e => e.description().to_owned(),
    }
}

pub fn format_error(interval: Interval, error_code: ErrorKind) -> ErrorInfo {
    let message = get_error_message(error_code);
    ErrorInfo { interval, message }
}