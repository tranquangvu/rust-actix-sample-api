#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate actix;
extern crate actix_web;

pub mod routes;
pub mod config;
pub mod models;
pub mod handlers;
pub mod app;

fn main() {
  app::boot();
}
