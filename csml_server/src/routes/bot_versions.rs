use actix_web::{post, get, delete, web, HttpResponse};
use csml_engine::{
  create_bot_version, get_bot_by_version_id, get_bot_versions, get_last_bot_version,
  delete_all_bot_versions, delete_bot_version_id
};
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};
use std::thread;


/*
 * create bot version
 *
 * {"statusCode": 200,"body": {"version_id": String} }
 *
 */
#[post("/bots")]
pub async fn add_bot_version(body: web::Json<CsmlBot>) -> HttpResponse {
  let bot = body.to_owned();

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
pub async fn get_bot_latest_version(path: web::Path<BotIdPath>) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();

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
) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();

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
pub async fn get_bot_latest_versions(path: web::Path<BotIdPath>, query: web::Query<GetBotVersionsQuery>) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();
  let limit = query.limit.to_owned();
  let pagination_key = match query.pagination_key.to_owned() {
    Some(pagination_key) if pagination_key == "" => None,
    Some(pagination_key) => Some(pagination_key),
    None => None,
  };

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
) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();
  let version_id = path.version_id.to_owned();

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
) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();
  let version_id = path.version_id.to_owned();

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
