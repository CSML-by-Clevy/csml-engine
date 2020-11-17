use crate::format_response;
use csml_engine::start_conversation;
use csml_engine::data::CsmlRequest;
use csml_interpreter::data::{csml_bot::CsmlBot};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
// use ureq::json;


#[derive(Debug, Serialize, Deserialize)]
struct SnsConfirmationRequest {
  #[serde(rename = "SubscribeURL")]
  subscribe_url: String,
}

fn confirm_subscription(body: &str) -> serde_json::Value {

    let val: SnsConfirmationRequest = match serde_json::from_str(body) {
        Ok(res) => res,
        Err(_) => return serde_json::json!("Request body can not be properly parsed"),
    };

    println!("SNS SubscribeURL: {}", val.subscribe_url);

    match ureq::get(&val.subscribe_url).call().ok() {
        true => serde_json::json!(true),
        false => serde_json::json!("Impossible to reach SubscribeURL"), // error 400
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RunRequest {
    bot: CsmlBot,
    event: CsmlRequest,
}

#[derive(Debug, Serialize, Deserialize)]
struct SnsMessage {
    #[serde(rename = "Message")]
    message: String,
}

fn handle_notification(body: &str) -> serde_json::Value {

    // All requests with an invalid should return a 200 code,
    // as we don't want the SNS event to be retried (same result).
    // Ideally, it should however raise an error on some logging/monitoring system
    let sns: SnsMessage = match serde_json::from_str(body) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("SNS request notification parse error: {:?}", err);
            return format_response(400, serde_json::json!("Request body can not be properly parsed"))
        },
    };

    // sns message is itself a JSON encoded string containing the actual CSML request
    let csml_request: RunRequest = match serde_json::from_str(&sns.message) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("SNS message notification parse error: {:?}", err);
            return format_response(400, serde_json::json!("Request body is not a valid CSML request"))
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

    let res = start_conversation(request, bot);

    match res {
        Ok(data) => format_response(200, serde_json::json!(data)),
        Err(err) => {
            eprintln!("EngineError: {:?}", err);
            return format_response(400, serde_json::json!(format!("EngineError: {:?}", err)))
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
pub fn handler(headers: serde_json::Value, body: String) -> serde_json::Value {

    // See AWS SNS docs for specification of how this endpoint is called for http/https notification event types:
    // https://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.prepare.html#http-subscription-confirmation-json
    match headers.get("x-amz-sns-message-type") {
        Some(val) if val == "SubscriptionConfirmation" => confirm_subscription(&body),
        Some(val) if val == "Notification" => handle_notification(&body),
        // other scenarios inclure unsubscribe requests and invalid/non-SNS requests
        _ => format_response(400, serde_json::json!("Bad request"))
    }
}
