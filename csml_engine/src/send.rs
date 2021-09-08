use crate::data::{ConversationInfo};

fn format_and_transfer(callback_url: &str, msg: serde_json::Value) {
    let mut request = ureq::post(callback_url);

    request.set("Accept", "application/json");
    request.set("Content-Type", "application/json");

    let response = request.send_json(msg);

    if let Some(err) = response.synthetic_error() {
        eprintln!("callback_url call failed: {:?}", err.body_text());
    }
}

/**
 * If a callback_url is defined, we must send each message to its endpoint as it comes.
 * Otherwise, just continue!
 */
pub fn send_to_callback_url(c_info: &mut ConversationInfo, msg: serde_json::Value) {
    let callback_url = match &c_info.callback_url {
        Some(callback_url) => callback_url,
        None => return,
    };

    format_and_transfer(callback_url, msg)
}
