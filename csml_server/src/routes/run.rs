use actix_web::{web, HttpResponse};
use csml_manager::start_conversation;
use csml_manager::data::CsmlData;
use csml_interpreter::data::{csml_bot::CsmlBot, Client};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestRun {
  bot: CsmlBot,
  event: RequestEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RequestEvent {
  request_id: String,
  client: Client,
  callback_url: Option<String>,
  payload: serde_json::Value,
  metadata: serde_json::Value,
}

pub async fn handler(body: web::Json<RequestRun>) -> HttpResponse {
  let event = &body.event;
  let bot = &body.bot;
  let data = CsmlData {
    request_id: event.request_id.clone(),
    client: event.client.clone(),
    callback_url: event.callback_url.clone(),
    payload: event.payload.clone(),
    metadata: {
      match event.metadata.clone() {
          Value::Null => json!({}),
          val => val,
      }
    },
    bot: bot.clone(),
  };

  let res = match start_conversation(
    json!(&event),
    data
  ) {
      Err(err) => {
        eprintln!("ManagerError: {:?}", err);
        HttpResponse::InternalServerError().finish()
      },
      Ok(obj) => {
        HttpResponse::Ok().json(obj)
      },
  };

  res
}
