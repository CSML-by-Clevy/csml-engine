use crate::data::position::Position;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub position: Position,
    pub message: String,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl ErrorInfo {
    pub fn new(position: Position, message: String) -> Self {
        Self { position, message }
    }
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl ErrorInfo {
    pub fn format_error(&self) -> String {
        format!(
            "{} at line {}, column {} at flow [{}]",
            self.message,
            self.position.interval.start_line,
            self.position.interval.start_column,
            self.position.flow,
        )
    }
}

// TODO: this is a tmp solution
impl From<std::io::Error> for ErrorInfo {
    fn from(e: std::io::Error) -> Self {
        Self {
            position: Position::default(),
            message: e.to_string(),
        }
    }
}

// TODO: this is a tmp solution
impl From<serde_json::Error> for ErrorInfo {
    fn from(e: serde_json::Error) -> Self {
        Self {
            position: Position::default(),
            message: e.to_string(),
        }
    }
}

impl From<uuid::Error> for ErrorInfo {
    fn from(e: uuid::Error) -> Self {
        Self {
            position: Position::default(),
            message: e.to_string(),
        }
    }
}

impl From<std::time::SystemTimeError> for ErrorInfo {
    fn from(e: std::time::SystemTimeError) -> Self {
        Self {
            position: Position::default(),
            message: e.to_string(),
        }
    }
}
