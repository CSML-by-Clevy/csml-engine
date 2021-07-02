
pub fn validate_api_key(req: &actix_web::HttpRequest) -> Option<String> {
    let api_key = match std::env::var("API_KEY") {
      Ok(val) if !val.is_empty() => val,
      _ => return None
    };

    let vec = api_key.split(',').collect::<Vec<&str>>();

    match req.headers().get("X-Api-Key") {
      Some(val) => {
        let val = val.to_str().unwrap_or("");
        if val.is_empty() || !vec.contains(&val) {
          eprintln!("EngineError: wrong X-Api-Key value [{}]", val);
          return Some(val.to_owned())
        }

        None
      },
      None => {
        eprintln!("EngineError: missing X-Api-Key in env");
        Some("".to_owned())
      }
    }
}