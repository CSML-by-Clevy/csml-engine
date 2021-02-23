use actix_web::{post, web, HttpResponse};
use csml_engine::{validate_bot, CsmlResult};
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ValidateBotResponse {
  valid: bool,
  // #[serde(skip_serializing_if = "Option::is_none")]
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
  step: String,
  start_line: u32,
  start_column: u32,
  end_line: Option<u32>,
  end_column: Option<u32>,
  message: String,
}

#[post("/validate")]
pub async fn handler(body: web::Json<CsmlBot>) -> HttpResponse {
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
          step: error_info.position.step.clone(),
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

  HttpResponse::Ok().json(response)
}
