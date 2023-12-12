# Thingelo API
Rust + Axum REST API for the **Thingelo** site.

## Dependencies
* Rust
* Cargo
* PostgreSQL

## How To Run Locally
* Configure a PostgreSQL server compatible with the **.env** file. You may, overwrite the .env properties by exporting your own variables with the same name.
* Run the application with **Cargo**.
```bash
cargo run
```
API hosted on port 8080.


## How To Build
```
cargo build --release
```
API executable will be stored in **target/release/rankit.exe**