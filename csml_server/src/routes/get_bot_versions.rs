use actix_web::{post, web, HttpResponse};
use csml_engine::{get_bot_versions};
use serde::{Deserialize, Serialize};
use std::thread;


#[derive(Debug, Serialize, Deserialize)]
pub struct GetBotVersionsRequest {
  bot_id: String,
}

#[post("/get_bot_versions")]
pub async fn handler(body: web::Json<GetBotVersionsRequest>) -> HttpResponse {
  let bot_id = body.bot_id.to_owned();

  let res = thread::spawn(move || {
    get_bot_versions(&bot_id)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}