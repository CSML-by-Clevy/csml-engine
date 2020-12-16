use actix_web::{post, web, HttpResponse};
use csml_engine::{get_last_bot_version};
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLastBotRequest {
  bot_id: String,
}

#[post("/get_last_bot_version")]
pub async fn handler(body: web::Json<GetLastBotRequest>) -> HttpResponse {
  let bot_id = body.bot_id.to_owned();
  

  let res = thread::spawn(move || {
    get_last_bot_version(&bot_id)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}