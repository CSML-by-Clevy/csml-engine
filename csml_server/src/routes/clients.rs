use actix_web::{delete, web, HttpResponse};
use csml_interpreter::data::{Client};
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientQuery {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,
}

/*
* Delete all data for a given Client
*
* {"statusCode": 204}
*
*/
#[delete("/clients)")]
pub async fn delete_client(query: web::Query<ClientQuery>) -> HttpResponse {
    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

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
