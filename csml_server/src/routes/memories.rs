use actix_web::{post, delete, get, web, HttpResponse};
use csml_interpreter::data::{Client};
use serde::{Deserialize, Serialize};
use std::thread;
use crate::routes::tools::validate_api_key;

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
 * Create client memory
 *
 * {"statusCode": 201}
 *
 */
#[post("/memories")]
pub async fn create_client_memory(
    query: web::Query<ClientQuery>,
    body: web::Json<Memory>,
    req: actix_web::HttpRequest
) -> HttpResponse {
    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    if let Some(value) = validate_api_key(&req) {
        return HttpResponse::Forbidden().finish()
    }

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
#[delete("/memories/{key}")]
pub async fn delete_memory(
    path: web::Path<MemoryKeyPath>,
    query: web::Query<ClientQuery>,
    req: actix_web::HttpRequest
) -> HttpResponse {
    let memory_key = path.key.to_owned();

    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    if let Some(value) = validate_api_key(&req) {
        return HttpResponse::Forbidden().finish()
    }

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

/**
 * Delete a client's full memory
 *
 * {"statusCode": 204}
 *
 */
#[delete("/memories")]
pub async fn delete_memories(query: web::Query<ClientQuery>, req: actix_web::HttpRequest) -> HttpResponse {
    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    if let Some(value) = validate_api_key(&req) {
        return HttpResponse::Forbidden().finish()
    }

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
 * Get a specific key in client memory
 *
 */
#[get("/memories/{key}")]
pub async fn get_memory(path: web::Path<MemoryKeyPath>, query: web::Query<ClientQuery>, req: actix_web::HttpRequest) -> HttpResponse {
    let memory_key = path.key.to_owned();

    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    if let Some(value) = validate_api_key(&req) {
        return HttpResponse::Forbidden().finish()
    }

    let res = thread::spawn(move || {
        csml_engine::get_client_memory(&client, &memory_key)
    }).join().unwrap();

    match res {
        Ok(memory) => HttpResponse::Ok().json(memory),
        Err(err) => {
            eprintln!("EngineError: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/**
* Get a client's full memory
*
*/
#[get("/memories")]
pub async fn get_memories(query: web::Query<ClientQuery>, req: actix_web::HttpRequest) -> HttpResponse {
    let client = Client {
        user_id: query.user_id.clone(),
        channel_id: query.channel_id.clone(),
        bot_id: query.bot_id.clone(),
    };

    if let Some(value) = validate_api_key(&req) {
        return HttpResponse::Forbidden().finish()
    }

    let res = thread::spawn(move || {
        csml_engine::get_client_memories(&client)
    }).join().unwrap();

    match res {
    Ok(memory) => HttpResponse::Ok().json(memory),
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
    async fn test_memories() {
        let mut app = test::init_service(
            App::new()
                    .service(get_memories)
                    .service(delete_memories)
                    .service(create_client_memory)
        ).await;

        let (user_id, channel_id, bot_id) = ("test", "memories-channel", "botid");

        let resp = test::TestRequest::delete()
                    .uri(&format!("/memories?user_id={}&channel_id={}&bot_id={}", user_id, channel_id, bot_id))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::TestRequest::post()
                    .uri(&format!("/memories?user_id={}&channel_id={}&bot_id={}", user_id, channel_id, bot_id))
                    .set_json(
                        &serde_json::json!({
                                "key": "val",
                                "value": 42
                        })
                    )
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::CREATED);


        let resp = test::TestRequest::get()
                    .uri(&format!("/memories?user_id={}&channel_id={}&bot_id={}", user_id, channel_id, bot_id))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body: serde_json::Value = serde_json::from_str(resp.response().body().as_str()).unwrap();

        assert_eq!(
            (body[0]["key"].clone(), body[0]["value"].clone()),
            (serde_json::json!("val"), serde_json::json!(42))
        );
    }

    #[actix_rt::test]
    async fn test_memory() {
        let mut app = test::init_service(
            App::new()
                    .service(get_memory)
                    .service(delete_memory)
                    .service(create_client_memory)
        ).await;

        let (user_id, channel_id, bot_id) = ("test", "memory-channel", "botid");
        let key = "val";

        let resp = test::TestRequest::delete()
                    .uri(&format!("/memories/val?user_id={}&channel_id={}&bot_id={}", user_id, channel_id, bot_id))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::TestRequest::post()
                    .uri(&format!("/memories?user_id={}&channel_id={}&bot_id={}", user_id, channel_id, bot_id))
                    .set_json(
                        &serde_json::json!({
                                "key": key,
                                "value": 42
                        })
                    )
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::CREATED);

        let resp = test::TestRequest::get()
                    .uri(&format!("/memories/{}?user_id={}&channel_id={}&bot_id={}", key, user_id, channel_id, bot_id))
                    .send_request(&mut app).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body: serde_json::Value = serde_json::from_str(resp.response().body().as_str()).unwrap();

        assert_eq!(
            (body["key"].clone(), body["value"].clone()),
            (serde_json::json!("val"), serde_json::json!(42))
        );
    }
}
