
use actix_web::{web, App, HttpServer};

pub fn boot() {
  let sys = actix::System::new("mystore");

  crate::routes::config();

  println!("Started http server: 127.0.0.1:8000");
  let _ = sys.run();
}