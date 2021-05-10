use actix_web::{post, delete, web, HttpResponse};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    key: String,
    value: serde_json::Value,
}


/**
 * Delete a client's full memory
 *
 * {"statusCode": 204}
 *
 */
#[delete("/memories")]
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

/**
 * Create client memory
 *
 * {"statusCode": 201}
 *
 */
#[post("/memories")]
pub async fn create_client_memory(query: web::Query<ClientQuery>, body: web::Json<Memory>) -> HttpResponse {
    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    let res = thread::spawn(move || {
        csml_engine::create_client_memory(&client, body.key.to_owned(), body.value.to_owned())
    }).join().unwrap();

    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            eprintln!("EngineError: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/**
 * Delete a specific key in client memory
 *
 * {"statusCode": 204}
 *
 */
#[delete("/memories/{key})")]
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

