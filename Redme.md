# diesel_rust_db_ex

## Configs:

* **Cargo.toml**:

`[dependencies]
diesel = { version = "2.0", features = ["postgres"] }
dotenvy = "0.15"
tokio = { version = "1", features = ["full"] }`


* **.env**:  

`DATABASE_URL=postgres://<user_name>:<password>@<host>:<port>/<db_name>`

