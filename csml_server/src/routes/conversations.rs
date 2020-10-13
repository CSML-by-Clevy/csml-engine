use actix_web::{post, web, HttpResponse};
use csml_engine::{user_close_all_conversations, get_open_conversation, Client};

#[post("/conversations/open")]
pub async fn get_open(body: web::Json<Client>) -> HttpResponse {

  let res = std::thread::spawn(move || {
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

  let res = std::thread::spawn(move || {
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
