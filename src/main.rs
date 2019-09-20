pub mod schema;
pub mod db_connection;
pub mod models;
pub mod handlers;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate actix;
extern crate actix_web;

use actix_web::{web, App, HttpServer};

fn main() {
  let sys = actix::System::new("mystore");

  HttpServer::new(|| { App::new()
    .service(
      web::resource("/").route(web::get().to_async(handlers::home::index))
    )
    .service(
      web::resource("/products")
        .route(web::get().to_async(handlers::products::index))
        .route(web::post().to_async(handlers::products::create))
    )
  })
  .bind("127.0.0.1:8000")
  .unwrap()
  .start();

  println!("Started http server: 127.0.0.1:8000");
  let _ = sys.run();
}
