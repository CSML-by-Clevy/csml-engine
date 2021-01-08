use actix_web::{post, web, HttpResponse};
use csml_engine::{get_bot_versions};
use serde::{Deserialize, Serialize};
use std::thread;


#[derive(Debug, Serialize, Deserialize)]
pub struct GetBotVersionsRequest {
  bot_id: String,
  limit: Option<i64>,
  last_key: Option<String>,
}

/*
* Get the last 20 versions of the bot if no limit is set
*
* {"statusCode": 200,"body": Vec<Bot>}
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
#[post("/get_bot_versions")]
pub async fn handler(body: web::Json<GetBotVersionsRequest>) -> HttpResponse {
  let bot_id = body.bot_id.to_owned();
  let limit = body.limit.to_owned();
  let last_key = body.last_key.to_owned();

  let res = thread::spawn(move || {
    get_bot_versions(&bot_id, limit, last_key)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}