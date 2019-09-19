use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().json("Hello World")
}

fn main() {
  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(index))
  })
  .bind("127.0.0.1:8000").expect("Can not bin to port 8000")
  .run()
  .unwrap();
}
