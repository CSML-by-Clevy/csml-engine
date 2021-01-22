use actix_web::{post, web, HttpResponse};
use csml_engine::{start_conversation};
use csml_engine::data::{RunRequest};
use serde_json::{Value, json};
use std::thread;

#[post("/run")]
pub async fn handler(body: web::Json<RunRequest>) -> HttpResponse {
  let mut request = body.event.to_owned();

  let bot_opt = match body.get_bot_opt() {
    Ok(bot_opt) => bot_opt,
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      return HttpResponse::BadRequest().finish()
    }
  };

  // request metadata should be an empty object by default
  request.metadata = match request.metadata {
    Value::Null => json!({}),
    val => val,
  };

  let res = thread::spawn(move || {
    start_conversation(request, bot_opt)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}
