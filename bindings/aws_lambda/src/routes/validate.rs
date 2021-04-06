use csml_engine::{validate_bot, CsmlResult};
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};

use lambda_runtime::{error::HandlerError};

#[derive(Debug, Serialize, Deserialize)]
struct ValidateBotResponse {
    valid: bool,
    errors: Vec<ValidationError>,
}

impl ValidateBotResponse {
    fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidationError {
    flow: String,
    start_line: u32,
    start_column: u32,
    end_line: Option<u32>,
    end_column: Option<u32>,
    message: String,
}

pub fn handler(body: CsmlBot) -> Result<serde_json::Value, HandlerError> {
    let response = match validate_bot(body.clone()) {
        CsmlResult {
            flows: _,
            warnings: _,
            errors: None,
        } => {
            ValidateBotResponse::new()
        }

        CsmlResult {
        flows: _,
        warnings: _,
        errors: Some(errors),
        } => {
            let mut errors_array = Vec::new();
            for (_, error_info) in errors.iter().enumerate() {
                errors_array.push(ValidationError {
                    flow: error_info.position.flow.clone(),
                    start_line: error_info.position.interval.start_line,
                    start_column: error_info.position.interval.start_column,
                    end_line: error_info.position.interval.end_line,
                    end_column: error_info.position.interval.end_column,
                    message: error_info.message.clone(),
                })
            }
            ValidateBotResponse {
                valid: false,
                errors: errors_array,
            }
        }
    };

    Ok(serde_json::json!(
        {
            "isBase64Encoded": false,
            "statusCode": 200,
            "headers": { "Content-Type": "application/json" },
            "body": serde_json::json!(response).to_string()
        }
    ))
}
