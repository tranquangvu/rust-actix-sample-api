use crate::schema::products;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
  pub id: i32,
  pub name: String,
  pub stock: f64,
  pub price: Option<i32>
}

#[derive(Insertable, Deserialize)]
#[table_name="products"]
pub struct NewProduct {
  pub name: String,
  pub stock: f64,
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

impl NewProduct {
  pub fn create(&self) -> Result<Product, diesel::result::Error> {
    use diesel::RunQueryDsl;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();
    diesel::insert_into(products::table)
      .values(self)
      .get_result(&connection)

  }
}

impl Product {
  pub fn find(id: &i32) -> Result<Product, diesel::result::Error> {
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();
    products::table.find(id).first(&connection)
  }

  pub fn destroy(id: &i32) -> Result<(), diesel::result::Error> {
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use crate::schema::products::dsl;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();

    diesel::delete(dsl::products.find(id)).execute(&connection)?;
    Ok(())
  }
}
