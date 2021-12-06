use crate::data::{
    literal::{Literal, create_error_info},
    position::Position,
};

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub position: Position,
    pub message: String,
    pub additional_info: Option<HashMap<String, Literal>>
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl ErrorInfo {
    pub fn new(
        position: Position,
        message: String,
    ) -> Self {
        let error_info = create_error_info(&message, position.interval);

        Self { position, message, additional_info: Some(error_info)}
    }

    pub fn add_info(&mut self, key: &str, value: Literal) {
        match self.additional_info {
            Some(ref mut map) => {
                map.insert(key.to_owned(), value);
            }
            None => {
                let mut info = HashMap::new();
                info.insert(key.to_owned(),value);

                self.additional_info = Some(info);
            }
        }
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

impl From<std::io::Error> for ErrorInfo {
    fn from(e: std::io::Error) -> Self {
        Self {
            position: Position::default(),
            message: e.to_string(),
            additional_info: None,
        }
    }
}

impl From<serde_json::Error> for ErrorInfo {
    fn from(e: serde_json::Error) -> Self {
        Self {
            position: Position::default(),
            message: e.to_string(),
            additional_info: None,
        }
    }
}

impl From<uuid::Error> for ErrorInfo {
    fn from(e: uuid::Error) -> Self {
        Self {
            position: Position::default(),
            message: e.to_string(),
            additional_info: None,
        }
    }
}

impl From<std::time::SystemTimeError> for ErrorInfo {
    fn from(e: std::time::SystemTimeError) -> Self {
        Self {
            position: Position::default(),
            message: e.to_string(),
            additional_info: None,
        }
    }
}
