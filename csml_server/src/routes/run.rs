use actix_web::{web, HttpResponse};
use csml_manager::{data::CsmlData, start_conversation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct CsmlRunBody {
  bot: serde_json::Value,
  event: serde_json::Value,
}

pub async fn handler(body: web::Json<CsmlRunBody>) -> HttpResponse {

  let data = CsmlData {
    request_id: body.event["request_id"].as_str().unwrap().to_owned(),
    client: serde_json::from_value(body.event["client"].clone()).unwrap(),
    callback_url: {
        match body.event["callback_url"].clone() {
          Value::Null => None,
          val => Some(val.as_str().unwrap().to_owned()),
        }
    },
    payload: serde_json::from_value(body.event["payload"].clone()).unwrap(),
    metadata: {
        match body.event["metadata"].clone() {
            Value::Null => json!({}),
            val => val,
        }
    },
    bot: serde_json::from_value(body.bot.clone()).unwrap(),
  };

  let res: HttpResponse = match start_conversation(body.event.clone(), data) {
      Err(err) => {
        eprintln!("notok: {:?}", err);
        // TODO: handle manager errors gracefully
        panic!("{:?}", err);
      },
      Ok(obj) => {
        eprintln!("ok   : {:?}", obj);
        HttpResponse::Ok().json(obj)
      },
  };

  res
}
