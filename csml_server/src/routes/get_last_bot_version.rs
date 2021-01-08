use actix_web::{post, web, HttpResponse};
use csml_engine::{get_last_bot_version};
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLastBotRequest {
  bot_id: String,
}

/*
* get last bot version
*
* {"statusCode": 200,"body": Bot}
*
* BOT = {
*  "version_id": String,
*  "id": String,
*  "name": String,
*  "custom_components": Option<String>,
*  "default_flow": String
*  "engine_version": String
*  "created_at": String
* }
*/
#[post("/get_last_bot_version")]
pub async fn handler(body: web::Json<GetLastBotRequest>) -> HttpResponse {
  let bot_id = body.bot_id.to_owned();
  

  let res = thread::spawn(move || {
    get_last_bot_version(&bot_id)
  }).join().unwrap();

  match res {
    Ok(Some(bot_version)) => HttpResponse::Ok().json(bot_version.flatten()),
    Ok(None) => HttpResponse::NotFound().finish(),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}