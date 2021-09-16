use actix_web::{delete, post, web, HttpResponse};
use csml_interpreter::data::{Client};
use serde::{Deserialize, Serialize};
use std::thread;
use crate::routes::tools::validate_api_key;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientQuery {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotIdPath {
    bot_id: String
}

/*
* Delete all data for a given Client
*
* {"statusCode": 204}
*
*/
#[delete("/data/clients")]
pub async fn delete_client(query: web::Query<ClientQuery>, req: actix_web::HttpRequest) -> HttpResponse {
    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    if let Some(_value) = validate_api_key(&req) {
        return HttpResponse::Forbidden().finish()
    }

    let res = thread::spawn(move || {
        csml_engine::delete_client(&client)
    }).join().unwrap();

    match res {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            eprintln!("EngineError: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/**
 * Delete all bot data
 *
 * {"statusCode": 204}
 *
 */
#[delete("/data/bots/{bot_id}")]
pub async fn delete_bot(path: web::Path<BotIdPath>, req: actix_web::HttpRequest) -> HttpResponse {

    if let Some(_value) = validate_api_key(&req) {
        return HttpResponse::Forbidden().finish()
    }

    let res = thread::spawn(move || {
        csml_engine::delete_all_bot_data(&path.bot_id)
    }).join().unwrap();

    match res {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            eprintln!("EngineError: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}


/**
 * Delete all expired data in db (Conversation, Messages, Memory and State)
 *
 * {"statusCode": 204}
 *
 */
#[post("/data/cleanup")]
pub async fn delete_expired_data() -> HttpResponse {

    let res = thread::spawn(move || {
        csml_engine::delete_expired_data()
    }).join().unwrap();

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("EngineError: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
   }
}
