use actix_web::{get, web, HttpResponse};
use csml_engine::{Client};
use serde::{Deserialize, Serialize};
use std::thread;


#[derive(Debug, Serialize, Deserialize)]
pub struct ClientQuery {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,
}

#[get("/conversations")]
pub async fn get_client_current_state(query: web::Query<ClientQuery>) -> HttpResponse {

  let client = Client {
    bot_id: query.bot_id.to_owned(),
    channel_id: query.channel_id.to_owned(),
    user_id: query.user_id.to_owned()
  };

  let res = thread::spawn(move || {
    csml_engine::get_current_state(&client)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
        eprintln!("EngineError: {:?}", err);
        HttpResponse::InternalServerError().finish()
    }
  }
}