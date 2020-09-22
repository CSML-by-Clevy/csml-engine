use actix_web::{web, HttpResponse};
use csml_engine::start_conversation;
use csml_engine::data::CsmlRequest;
use csml_interpreter::data::{csml_bot::CsmlBot};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestRun {
  bot: CsmlBot,
  event: CsmlRequest,
}


pub async fn handler(body: web::Json<RequestRun>) -> HttpResponse {
  let bot = body.bot.to_owned();
  let mut request = body.event.to_owned();

  // request metadata should be an empty object by default
  request.metadata = match request.metadata {
    Value::Null => json!({}),
    val => val,
  };

  let res = thread::spawn(move || {
    start_conversation(request, bot)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }

}
