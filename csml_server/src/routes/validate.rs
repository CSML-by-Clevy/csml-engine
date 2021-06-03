use actix_web::{post, web, HttpResponse};
use csml_engine::{validate_bot, CsmlResult};
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use actix_web::http::{StatusCode};

    #[actix_rt::test]
    async fn test_validate() {
        let mut app = test::init_service(
            App::new()
                    .service(handler)
        ).await;

        let resp = test::TestRequest::post()
                    .uri(&format!("/validate"))
                    .set_json(&serde_json::json!({
                          "id": "test_run",
                          "name": "test_run",
                          "flows": [
                            {
                              "id": "Default",
                              "name": "Default",
                              "content": "start: say \"Hello\" goto end",
                              "commands": [],
                            }
                          ],
                          "default_flow": "Default",
                    }))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}