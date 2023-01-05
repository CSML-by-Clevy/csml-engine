use crate::routes::tools::validate_api_key;
use actix_web::{get, web, HttpResponse};
use csml_engine::Client;
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientQuery {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,
}

#[get("/state")]
pub async fn get_client_current_state(
    query: web::Query<ClientQuery>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    let client = Client {
        bot_id: query.bot_id.to_owned(),
        channel_id: query.channel_id.to_owned(),
        user_id: query.user_id.to_owned(),
    };

    if let Some(value) = validate_api_key(&req) {
        eprintln!("AuthError: {:?}", value);
        return HttpResponse::Forbidden().finish();
    }

    let res = thread::spawn(move || csml_engine::get_current_state(&client))
        .join()
        .unwrap();

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
    use actix_web::http::StatusCode;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_messages() {
        let mut app = test::init_service(App::new().service(get_client_current_state)).await;

        let (user_id, channel_id, bot_id) = ("test", "state-channel", "botid");

        let resp = test::TestRequest::get()
            .uri(&format!(
                "/state?user_id={}&channel_id={}&bot_id={}",
                user_id, channel_id, bot_id
            ))
            .send_request(&mut app)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
