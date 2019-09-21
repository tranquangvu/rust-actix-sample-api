# Rust Sample API
Practice rust language by building sample API by actix framework

## Install
* Make sure you have installed Rust
  ```
    curl https://sh.rustup.rs -sSf | sh
    source $HOME/.cargo/env
    export PATH="$HOME/.cargo/bin:$PATH"
  ```
* Clone this source code
* Add `.env` (Refer from `.env.example`)
* Build app
  ```
    cargo build
  ```
* Setup `diesel_cli` and init database
  ```
    cargo install diesel_cli
    diesel setup
    diesel migration run
  ```
* Make some seed data
  ```
    psql -U postgres -d mystore -c "INSERT INTO products(name, stock, price) VALUES ('shoes', 10.0, 100); INSERT INTO products(name, stock, price) VALUES ('hats', 5.0, 50);"
  ```
* Start server
  ```
    cargo run
  ```
  Server run on `127.0.0.1:8000`
* Example requests
  - Get all products
    ```
      curl http://127.0.0.1:8000/products
    ```
  - Create products
    ```
      curl http://127.0.0.1:8000/products -H "Content-Type: application/json" -d '{"name": "socks", "stock": 7, "price": 2}'
    ```
  - Show product
    ```
      curl http://127.0.0.1:8000/products/1
    ```
  - Update product
    ```
      curl -X PATCH http://127.0.0.1:8000/products/1 -H "Content-Type: application/json" -d '{"stock": 8}'
    ```
  - Delete product
    ```
      curl -X DELETE http://127.0.0.1:8000/products/1 -H "Content-Type: application/json"
    ```
