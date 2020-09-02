use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, http::header};
use actix_files as fs;

mod routes;

const MAX_BODY_SIZE: usize = 8_388_608; // 8MB


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
    .wrap(
      Cors::new()
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
          header::AUTHORIZATION,
          header::ACCEPT,
          header::CONTENT_TYPE
        ])
        .max_age(86_400) //24h
        .finish(),
      )
      .wrap(middleware::Logger::default())
      .data(web::JsonConfig::default().limit(MAX_BODY_SIZE))

      .service(
        fs::Files::new("/static", "./static")
          .use_last_modified(true)
      )

      .service(
        web::resource("/")
          .route(web::get().to(routes::index::get))
      )
      .service(
        web::resource("/validate")
          .route(web::post().to(routes::validate::handler))
      )
      .service(
        web::resource("/run")
          .route(web::post().to(routes::run::handler))
      )

  })
  .bind("0.0.0.0:5000")?
  .run()
  .await
}

