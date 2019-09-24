use actix_web::{middleware, web, App, HttpServer};

pub fn route_config(cfg: &mut web::ServiceConfig) {
  use crate::handlers;
  cfg.service(
    web::scope("/products")
      .service(
        web::resource("/").route(web::get().to_async(handlers::home::index))
      )
      .service(
        web::resource("/products")
          .route(web::get().to_async(handlers::products::index))
          .route(web::post().to_async(handlers::products::create))
      )
      .service(
        web::resource("/products/{id}")
          .route(web::get().to_async(handlers::products::show))
          .route(web::delete().to_async(handlers::products::destroy))
          .route(web::patch().to_async(handlers::products::update))
      ),
  );
}

pub fn config() {
  HttpServer::new(|| App::new()
      .configure(route_config)
      .wrap(middleware::Logger::default())
    )
    .bind("127.0.0.1:8000")
    .unwrap()
    .start();  
}