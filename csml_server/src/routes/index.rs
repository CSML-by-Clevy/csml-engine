use actix_web::HttpResponse;

pub async fn get() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(include_str!("../../static/index.html"))
}
