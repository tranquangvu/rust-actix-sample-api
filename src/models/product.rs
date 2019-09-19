use crate::schema::products;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
  pub id: i32,
  pub name: String,
  pub stock: f64,
  pub price: Option<i32>
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct {
  pub name: Option<String>,
  pub stock: Option<f64>,
  pub price: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
  pub fn list() -> Self {
    use diesel::RunQueryDsl;
    use diesel::QueryDsl;
    use crate::schema::products::dsl::*;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();
    let result = products
      .limit(10)
      .load::<Product>(&connection)
      .expect("Error loading products");

    ProductList(result)
  }
}
