use actix_web::{post, web, HttpResponse, HttpRequest, client};
use csml_engine::{start_conversation};
use csml_engine::data::{CsmlRequest, BotOpt};
use csml_interpreter::data::{csml_bot::CsmlBot};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
struct SnsConfirmationRequest {
  #[serde(rename = "SubscribeURL")]
  subscribe_url: String,
}

async fn confirm_subscription(body: &str) -> HttpResponse {

  let val: SnsConfirmationRequest = match serde_json::from_str(body) {
    Ok(res) => res,
    Err(_) => return HttpResponse::BadRequest().body("Request body can not be properly parsed"),
  };

  println!("SNS SubscribeURL: {}", val.subscribe_url);

  let http = client::Client::default();

  match http.get(val.subscribe_url.to_owned())
    .send()
    .await {
      Ok(_) => HttpResponse::Ok().finish(),
      Err(_) => HttpResponse::BadGateway().body("Impossible to reach SubscribeURL"),
  }

}

#[derive(Debug, Serialize, Deserialize)]
struct RunRequest {
  bot: BotOpt,
  event: CsmlRequest,
}

#[derive(Debug, Serialize, Deserialize)]
struct SnsMessage {
  #[serde(rename = "Message")]
  message: String,
}

async fn handle_notification(body: &str) -> HttpResponse {

  // All requests with an invalid should return a 200 code,
  // as we don't want the SNS event to be retried (same result).
  // Ideally, it should however raise an error on some logging/monitoring system
  let sns: SnsMessage = match serde_json::from_str(body) {
    Ok(res) => res,
    Err(err) => {
      eprintln!("SNS request notification parse error: {:?}", err);
      return HttpResponse::Ok().body("Request body can not be properly parsed");
    },
  };

  // sns message is itself a JSON encoded string containing the actual CSML request
  let csml_request: RunRequest = match serde_json::from_str(&sns.message) {
    Ok(res) => res,
    Err(err) => {
      eprintln!("SNS message notification parse error: {:?}", err);
      return HttpResponse::Ok().body("Request body is not a valid CSML request");
    },
  };

  // same behavior as /run requests
  let bot = csml_request.bot.to_owned();
  let mut request = csml_request.event.to_owned();

  // request metadata should be an empty object by default
  request.metadata = match request.metadata {
    Value::Null => json!({}),
    val => val,
  };

  let res = thread::spawn(move || {
    start_conversation(request, bot)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }

}

/**
 * Handle CSML requests asynchronously as AWS SNS messages.
 * This endpoint must handle both SNS message handling and SNS
 * HTTP/HTTPS endpoint subscription requests.
 * No message will be sent to the endpoint until the subscription
 * has been properly confirmed.
 */
#[post("/sns")]
pub async fn handler(req: HttpRequest, body: web::Bytes) -> HttpResponse {

  let body_string = match std::str::from_utf8(&body) {
    Ok(res) => res,
    Err(_) => return HttpResponse::BadRequest().body("Request body can not be properly parsed"),
  };

  // See AWS SNS docs for specification of how this endpoint is called for http/https notification event types:
  // https://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.prepare.html#http-subscription-confirmation-json
  let sns_type = req.head().headers().get("x-amz-sns-message-type");

  if let Some(val) = sns_type {
    if val == "SubscriptionConfirmation" {
      return confirm_subscription(&body_string).await;
    }
    if val == "Notification" {
      return handle_notification(&body_string).await;
    }
  };

  // other scenarios inclure unsubscribe requests and invalid/non-SNS requests
  return HttpResponse::BadRequest().finish();

}
