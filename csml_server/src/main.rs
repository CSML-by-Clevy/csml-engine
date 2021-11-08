use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, http::header};
use actix_files as fs;
use csml_engine::make_migrations;

mod routes;

const MAX_BODY_SIZE: usize = 8_388_608; // 8MB

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let server_port: String = match std::env::var("ENGINE_SERVER_PORT") {
    Ok(val) => val,
    Err(_) => "5000".to_owned(),
  };
  println!("CSML Server listening on port {}", server_port);

  // make migrations for PgSQL and do nothing for MongoDB and DynamoDB
  match make_migrations() {
    Ok(_) => (),
    Err(err) => panic!("PgSQL Migration ERROR: {:?}", err)
  };

  HttpServer::new(|| {
    App::new()
      .wrap(
        Cors::default()
          .send_wildcard()
          .allowed_methods(vec!["GET", "POST", "DELETE"])
          .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE
          ])
          .max_age(86_400) //24h
      )
      .wrap(middleware::Logger::default())
      .data(web::JsonConfig::default().limit(MAX_BODY_SIZE))

      .service(fs::Files::new("/static", "./static").use_last_modified(true))

      .service(routes::index::home)
      .service(routes::validate::handler)

      .service(routes::status::get_status)

      .service(routes::run::handler)

      .service(routes::sns::handler)

      .service(routes::bot_versions::add_bot_version)
      .service(routes::bot_versions::get_bot_version)
      .service(routes::bot_versions::get_bot_latest_version)
      .service(routes::bot_versions::get_bot_latest_versions)
      .service(routes::bot_versions::delete_bot_version)
      .service(routes::bot_versions::delete_bot_versions)

      .service(routes::conversations::get_open)
      .service(routes::conversations::close_user_conversations)
      .service(routes::conversations::get_client_conversations)

      .service(routes::memories::create_client_memory)
      .service(routes::memories::get_memories)
      .service(routes::memories::get_memory)
      .service(routes::memories::delete_memories)
      .service(routes::memories::delete_memory)

      .service(routes::messages::get_client_messages)

      .service(routes::state::get_client_current_state)

      .service(routes::data::delete_expired_data)
      .service(routes::data::delete_bot)
      .service(routes::data::delete_client)

  })
  .bind(format!("0.0.0.0:{}", server_port))?
  .run()
  .await
}

