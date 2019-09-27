use actix_web::{App, HttpServer};
use crate::config::routes;

pub fn boot() {
  let system = actix::System::new("mystore");

  HttpServer::new(|| {
    App::new()
      .configure(routes::init)
  })
  .bind("127.0.0.1:8000")
  .unwrap()
  .start();

  println!("Started http server: 127.0.0.1:8000");

  let _ = system.run();
}
