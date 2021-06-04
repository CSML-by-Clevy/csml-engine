use actix_web::{get, post, web, HttpResponse};
use csml_engine::{user_close_all_conversations, get_open_conversation, Client};
use serde::{Deserialize, Serialize};
use std::thread;

/**
 * If a conversation is open, return it.
 * Otherwise, return nothing
 */
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

/**
 * Close any open conversation
 */
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

/**
 * List all the conversations of a given client
 */
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use actix_web::http::{StatusCode};

    #[actix_rt::test]
    async fn test_get_open_conversations() {
        let mut app = test::init_service(
            App::new()
                    .service(get_open)
        ).await;

        let (user_id, channel_id, bot_id) = ("test", "open-conversations-channel", "botid");

        let resp = test::TestRequest::post()
                    .uri(&format!("/conversations/open"))
                    .set_json(&serde_json::json!({
                      "user_id": user_id,
                      "channel_id": channel_id,
                      "bot_id": bot_id
                    }))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_close_conversations() {
        let mut app = test::init_service(
            App::new()
                    .service(close_user_conversations)
        ).await;

        let (user_id, channel_id, bot_id) = ("test", "close-conversations-channel", "botid");

        let resp = test::TestRequest::post()
                    .uri(&format!("/conversations/close"))
                    .set_json(&serde_json::json!({
                      "user_id": user_id,
                      "channel_id": channel_id,
                      "bot_id": bot_id
                    }))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_conversations() {
        let mut app = test::init_service(
            App::new()
                    .service(get_client_conversations)
        ).await;

        let (user_id, channel_id, bot_id) = ("test", "conversations-channel", "botid");

        let resp = test::TestRequest::get()
                    .uri(&format!("/conversations?user_id={}&channel_id={}&bot_id={}", user_id, channel_id, bot_id))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}