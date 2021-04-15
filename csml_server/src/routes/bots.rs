use actix_web::{delete, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryKeyPath {
    bot_id: String
}

/*
* Delete all bot data
*
* {"statusCode": 204}
*
*/
#[delete("/bots/{bot_id}")]
pub async fn delete_bot(path: web::Path<MemoryKeyPath>,) -> HttpResponse {

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
