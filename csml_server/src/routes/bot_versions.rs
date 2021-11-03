use actix_web::{post, get, delete, web, HttpResponse};
use csml_engine::{
  create_bot_version, get_bot_by_version_id, get_bot_versions, get_last_bot_version,
  delete_all_bot_versions, delete_bot_version_id, fold_bot
};
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};
use std::thread;
use crate::routes::tools::validate_api_key;

/**
 * fold bot into a single flow
 *
 * {"statusCode": 200,"body": {"flow": String} }
 *
 */
#[post("/bots/fold")]
pub async fn make_bot_fold(body: web::Json<CsmlBot>, req: actix_web::HttpRequest) -> HttpResponse {
  let bot = body.to_owned();

  if let Some(value) = validate_api_key(&req) {
    eprintln!("AuthError: {:?}", value);
    return HttpResponse::Forbidden().finish()
  }

  let res = thread::spawn(move || {
    fold_bot(bot)
  }).join().unwrap();

  match res {
    Ok(flow) => HttpResponse::Created().json(serde_json::json!({"flow": flow})),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

/**
 * create bot version
 *
 * {"statusCode": 200,"body": {"version_id": String} }
 *
 */
#[post("/bots")]
pub async fn add_bot_version(body: web::Json<CsmlBot>, req: actix_web::HttpRequest) -> HttpResponse {
  let bot = body.to_owned();

  if let Some(value) = validate_api_key(&req) {
    eprintln!("AuthError: {:?}", value);
    return HttpResponse::Forbidden().finish()
  }

  let res = thread::spawn(move || {
    create_bot_version(bot)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Created().json(serde_json::json!(data)),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotIdPath {
  bot_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBotVersionsQuery {
  limit: Option<i64>,
  pagination_key: Option<String>,
}

/**
 * get the latest version of a given bot
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
#[get("/bots/{bot_id}")]
pub async fn get_bot_latest_version(path: web::Path<BotIdPath>, req: actix_web::HttpRequest) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();

  if let Some(value) = validate_api_key(&req) {
    eprintln!("AuthError: {:?}", value);
    return HttpResponse::Forbidden().finish()
  }

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

/**
 * Delete all versions of a given bot
 *
 * {"statusCode": 204}
 */
#[delete("/bots/{bot_id}")]
pub async fn delete_bot_versions(
  path: web::Path<BotIdPath>,
  req: actix_web::HttpRequest,
) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();

  if let Some(value) = validate_api_key(&req) {
    eprintln!("AuthError: {:?}", value);
    return HttpResponse::Forbidden().finish()
  }

  let res = thread::spawn(move || {
    delete_all_bot_versions(&bot_id)
  }).join().unwrap();

  match res {
    Ok(_) => HttpResponse::NoContent().finish(),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

/*
 * Get the last versions of the bot. This does not return the flows!
 * Limited to 20 versions if not specified
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
#[get("/bots/{bot_id}/versions")]
pub async fn get_bot_latest_versions(path: web::Path<BotIdPath>, query: web::Query<GetBotVersionsQuery>, req: actix_web::HttpRequest) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();
  let limit = query.limit.to_owned();
  let pagination_key = match query.pagination_key.to_owned() {
    Some(pagination_key) if pagination_key == "" => None,
    Some(pagination_key) => Some(pagination_key),
    None => None,
  };

  if let Some(value) = validate_api_key(&req) {
    eprintln!("AuthError: {:?}", value);
    return HttpResponse::Forbidden().finish()
  }

  let res = thread::spawn(move || {
    get_bot_versions(&bot_id, limit, pagination_key)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotVersionPath {
  bot_id: String,
  version_id: String,
}

/*
 * Retrieve a specific version of a bot
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
#[get("/bots/{bot_id}/versions/{version_id}")]
pub async fn get_bot_version(
  path: web::Path<BotVersionPath>
, req: actix_web::HttpRequest) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();
  let version_id = path.version_id.to_owned();

  if let Some(value) = validate_api_key(&req) {
    eprintln!("AuthError: {:?}", value);
    return HttpResponse::Forbidden().finish()
  }

  let res = thread::spawn(move || {
    get_bot_by_version_id(&version_id, &bot_id)
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

/*
 * Delete a specific version of a bot
 *
 * {"statusCode": 204}
 */
#[delete("/bots/{bot_id}/versions/{version_id}")]
pub async fn delete_bot_version(
  path: web::Path<BotVersionPath>
, req: actix_web::HttpRequest) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();
  let version_id = path.version_id.to_owned();

  if let Some(value) = validate_api_key(&req) {
    eprintln!("AuthError: {:?}", value);
    return HttpResponse::Forbidden().finish()
  }

  let res = thread::spawn(move || {
    delete_bot_version_id(&version_id, &bot_id)
  }).join().unwrap();

  match res {
    Ok(_) => HttpResponse::NoContent().finish(),
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
    use actix_web::body::{Body, ResponseBody};
    use actix_web::http::{StatusCode};
    trait BodyTest {
      fn as_str(&self) -> &str;
    }

    impl BodyTest for ResponseBody<Body> {
        fn as_str(&self) -> &str {
            match self {
                ResponseBody::Body(ref b) => match b {
                    Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                    _ => panic!(),
                },
                ResponseBody::Other(ref b) => match b {
                    Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                    _ => panic!(),
                },
            }
        }
    }

    #[actix_rt::test]
    async fn test_delete_bot_version() {
        let mut app = test::init_service(
            App::new()
                    .service(delete_bot_version)
        ).await;

        let ( bot_id, version_id) = ("botid", "60b872d1009d9aa600b108ea");

        let resp = test::TestRequest::delete()
                    .uri(&format!("/bots/{}/versions/{}", bot_id, version_id))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_rt::test]
    async fn test_delete_bot_versions() {
        let mut app = test::init_service(
            App::new()
                    .service(delete_bot_versions)
        ).await;

        let bot_id = "bot_versions";

        let resp = test::TestRequest::delete()
                    .uri(&format!("/bots/{}", bot_id))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_rt::test]
    async fn test_add_bot_version() {
        let mut app = test::init_service(
            App::new()
                    .service(add_bot_version)
        ).await;

        let resp = test::TestRequest::post()
                    .uri(&format!("/bots"))
                    .set_json(&serde_json::json!({
                        "id": "bot_id",
                        "name": "test",
                        "flows": [
                          {
                            "id": "Default",
                            "name": "Default",
                            "content": "start: say \"Hello\" goto end",
                            "commands": [],
                          }
                        ],
                        "default_flow": "Default",
                    }))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }


    #[actix_rt::test]
    async fn test_get_bot_latest_versions() {
        let mut app = test::init_service(
            App::new()
                    .service(get_bot_latest_versions)
        ).await;

        let bot_id = "botid";

        let resp = test::TestRequest::get()
                    .uri(&format!("/bots/{}/versions", bot_id))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_bot_version() {
        let mut app = test::init_service(
            App::new()
                    .service(get_bot_version)
                    .service(add_bot_version)

        ).await;

        let resp = test::TestRequest::post()
        .uri(&format!("/bots"))
        .set_json(&serde_json::json!({
            "id": "bot_id_test",
            "name": "bot_id_test",
            "flows": [
              {
                "id": "Default",
                "name": "Default",
                "content": "start: say \"Hello\" goto end",
                "commands": [],
              }
            ],
            "default_flow": "Default",
        }))
        .send_request(&mut app).await;

        let body: serde_json::Value = serde_json::from_str(resp.response().body().as_str()).unwrap();

        let (bot_id, bot_version) = ("bot_id_test", body["version_id"].as_str().unwrap());

        let resp = test::TestRequest::get()
                    .uri(&format!("/bots/{}/versions/{}", bot_id, bot_version))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
