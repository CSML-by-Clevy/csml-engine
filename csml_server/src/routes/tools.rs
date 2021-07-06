
pub fn validate_api_key(req: &actix_web::HttpRequest) -> Option<String> {
    let api_keys = match std::env::var("ENGINE_SERVER_API_KEYS") {
      Ok(val) if !val.is_empty() => val,
      _ => return None
    };

    let vec = api_keys.split(',').collect::<Vec<&str>>();

    match req.headers().get("X-Api-Key") {
      Some(val) => {
        let val = val.to_str().unwrap_or("");
        if val.is_empty() || !vec.contains(&val) {
          return Some(format!("Invalid X-Api-Key value [{}]", val))
        }
        None
      },
      None => {
        Some("Missing X-Api-Key in header".to_owned())
      }
    }
}
