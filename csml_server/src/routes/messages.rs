use actix_web::{get, web, HttpResponse};
use csml_interpreter::data::{Client};
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationIdPath {
    conversation_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClientInfoQuery {
    user_id: String,
    bot_id: String,
    channel_id: String,
    limit: Option<i64>,
    pagination_key: Option<String>,
}

/*
*
*/
#[get("/conversations/{conversation_id}/messages")]
pub async fn get_client_conversation_messages(path: web::Path<ConversationIdPath>, query: web::Query<GetClientInfoQuery>) -> HttpResponse {

    let client = Client {
        bot_id: query.bot_id.to_owned(),
        channel_id: query.channel_id.to_owned(),
        user_id: query.user_id.to_owned()
    };

    let conversation_id = path.conversation_id.to_owned();

    let limit = query.limit.to_owned();
    let pagination_key = match query.pagination_key.to_owned() {
        Some(pagination_key) if pagination_key == "" => None,
        Some(pagination_key) => Some(pagination_key),
        None => None,
    };

    let res = thread::spawn(move || {
        csml_engine::get_client_conversation_messages(&client, &conversation_id, limit, pagination_key)
    }).join().unwrap();

    match res {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
        eprintln!("EngineError: {:?}", err);
        HttpResponse::InternalServerError().finish()
        }
   }
}