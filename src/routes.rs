use actix_web::{web, App, HttpServer};

pub fn config() {
  HttpServer::new(|| { App::new()
    .service(
      web::resource("/").route(web::get().to_async(crate::handlers::home::index))
    )
    .service(
      web::resource("/products")
        .route(web::get().to_async(crate::handlers::products::index))
        .route(web::post().to_async(crate::handlers::products::create))
    )
    .service(
      web::resource("/products/{id}")
        .route(web::get().to_async(crate::handlers::products::show))
        .route(web::delete().to_async(crate::handlers::products::destroy))
        .route(web::patch().to_async(crate::handlers::products::update))
    )
  })
  .bind("127.0.0.1:8000")
  .unwrap()
  .start();  
}