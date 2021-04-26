use actix_web::{delete, web, HttpResponse};
use csml_interpreter::data::{Client};
use serde::{Deserialize, Serialize};
use std::thread;


#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryKeyPath {
    key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientQuery {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,
}

/*
* Delete client memory key
*
* {"statusCode": 204}
*
*/
#[delete("/client/memories")]
pub async fn delete_memories( query: web::Query<ClientQuery>) -> HttpResponse {
    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    let res = thread::spawn(move || {
        csml_engine::delete_client_memories(&client)
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
* Delete all client memories 
*
* {"statusCode": 204}
*
*/
#[delete("/client/memories/{key})")]
pub async fn delete_memory(path: web::Path<MemoryKeyPath>, query: web::Query<ClientQuery>) -> HttpResponse {
    let memory_key = path.key.to_owned();

    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    let res = thread::spawn(move || {
        csml_engine::delete_client_memory(&client, &memory_key)
    }).join().unwrap();

    match res {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            eprintln!("EngineError: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
