use actix_web::{post, web, HttpResponse};
use csml_engine::{start_conversation};
use csml_engine::data::{RunRequest};
use serde_json::{Value, json};
use std::thread;

#[post("/run")]
pub async fn handler(body: web::Json<RunRequest>) -> HttpResponse {
  let mut request = body.event.to_owned();

  let bot_opt = match body.get_bot_opt() {
    Ok(bot_opt) => bot_opt,
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      return HttpResponse::BadRequest().finish()
    }
  };

  // request metadata should be an empty object by default
  request.metadata = match request.metadata {
    Value::Null => json!({}),
    val => val,
  };

  let res = thread::spawn(move || {
    start_conversation(request, bot_opt)
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
    async fn test_run() {
        let mut app = test::init_service(
            App::new()
                    .service(handler)
        ).await;

        let resp = test::TestRequest::post()
                    .uri(&format!("/run"))
                    .set_json(&serde_json::json!({
                        "bot": {
                            "id": "test_run",
                            "name": "test_run",
                            "flows": [
                              {
                                "id": "Default",
                                "name": "Default",
                                "content": "start: say \"Hello\" goto end",
                                "commands": [],
                              }
                            ],
                            "default_flow": "Default",
                        },
                        "event": {
                            "request_id": "request_id",
                            "client": {
                                "user_id": "user_id",
                                "channel_id": "channel_id",
                                "bot_id": "test_run"
                            },
                            "payload": { 
                              "content_type": "text" ,
                              "content": {
                                "text": "toto"
                              }
                            },
                            "metadata": Value::Null,
                        },
                    }))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}