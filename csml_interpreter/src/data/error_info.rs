use crate::data::position::Position;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
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
            "{} at line {}, column {} in step [{}] from flow [{}]",
            self.message,
            self.position.interval.line,
            self.position.interval.column,
            self.position.step,
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
