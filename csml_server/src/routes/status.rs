use actix_web::{get, HttpResponse};
use std::thread;

/*
* Get Server status
*
* {"statusCode": 200}
*
*/
#[get("/status")]
pub async fn get_status() -> HttpResponse {

    let res = thread::spawn(move || {
        csml_engine::get_status()
    }).join().unwrap();

    match res {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            eprintln!("EngineError: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}