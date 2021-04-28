use actix_web::{get, post, web, HttpResponse};
use csml_engine::{user_close_all_conversations, get_open_conversation, Client};
use serde::{Deserialize, Serialize};
use std::thread;

#[post("/conversations/open")]
pub async fn get_open(body: web::Json<Client>) -> HttpResponse {

  let res = thread::spawn(move || {
    get_open_conversation(&body)
  }).join().unwrap();

  match res {
    Ok(Some(conversation)) => HttpResponse::Ok().json(conversation),
    Ok(None) => HttpResponse::Ok().finish(),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }

}

#[post("/conversations/close")]
pub async fn close_user_conversations(body: web::Json<Client>) -> HttpResponse {

  let res = thread::spawn(move || {
    user_close_all_conversations(body.clone())
  }).join().unwrap();

  match res {
    Ok(()) => HttpResponse::Ok().finish(),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClientInfoQuery {
  user_id: String,
  bot_id: String,
  channel_id: String,
  limit: Option<i64>,
  pagination_key: Option<String>,
}

#[get("/conversations")]
pub async fn get_client_conversations(query: web::Query<GetClientInfoQuery>) -> HttpResponse {

  let client = Client {
    bot_id: query.bot_id.to_owned(),
    channel_id: query.channel_id.to_owned(),
    user_id: query.user_id.to_owned()
  };

  let limit = query.limit.to_owned();
  let pagination_key = match query.pagination_key.to_owned() {
    Some(pagination_key) if pagination_key == "" => None,
    Some(pagination_key) => Some(pagination_key),
    None => None,
  };

  let res = thread::spawn(move || {
    csml_engine::get_client_conversations(&client, limit, pagination_key)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
    eprintln!("EngineError: {:?}", err);
    HttpResponse::InternalServerError().finish()
    }
  }
}