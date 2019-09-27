use actix_web::{web};
use crate::handlers;

pub fn init(service_config: &mut web::ServiceConfig) {
  service_config
    .service(
      web::resource("/")
        .route(web::get().to_async(handlers::home::index))
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
    );
}
