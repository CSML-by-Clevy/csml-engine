use actix_web::{web, HttpResponse};
use csml_manager::{user_close_all_conversations, get_open_conversation, Client};

pub async fn get_open(body: web::Json<Client>) -> HttpResponse {

  match get_open_conversation(&body) {
    Ok(Some(conversation)) => HttpResponse::Ok().json(conversation),
    Ok(None) => HttpResponse::Ok().finish(),
    Err(err) => {
      eprintln!("ManagerError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }

}

pub async fn close_user_conversations(body: web::Json<Client>) -> HttpResponse {

  match user_close_all_conversations(body.clone()) {
    Ok(()) => HttpResponse::Ok().finish(),
    Err(err) => {
      eprintln!("ManagerError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }

}
