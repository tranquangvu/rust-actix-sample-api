use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::product::ProductList;
use crate::models::product::NewProduct;
use crate::models::product::Product;

pub fn index(_req: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().json(ProductList::list())
}

pub fn create(new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
  new_product
    .create()
    .map(|product| HttpResponse::Ok().json(product))
    .map_err(|e| {
      HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
  Product::find(&id)
    .map(|product| HttpResponse::Ok().json(product))
    .map_err(|e| {
      HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub fn destroy(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
  Product::destroy(&id)
    .map(|_| HttpResponse::Ok().json(()))
    .map_err(|e| {
      HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub fn update(id: web::Path<i32>, new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
  Product::update(&id, &new_product)
    .map(|_| HttpResponse::Ok().json(()))
    .map_err(|e| {
      HttpResponse::InternalServerError().json(e.to_string())
    })
}
