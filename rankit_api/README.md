# Thingelo API
Rust + Axum REST API for the **Thingelo** site.

## Dependencies
* Rust
* Cargo
* PostgreSQL

## How to Run Locally
* Install Postgres for your system if not already installed.
* Create a database called **thingelo**.
* Export the environment variable **APP_DB** that connects to **thingelo**.
  See **.env** for the default connection string.
* Run the application in dev mode:
```bash
cargo run
```
The REST API will be hosted on port 8080.

## How to Populate Sample Data
To make the app easier to test manually, you can run a the **populate** bin crate via:
```bash
cargo run --bin populate
```
This will run API calls that seed the application with some initial data.


## How to Build
```
cargo build --release
```
This will compile the application in release mode.
API executable will be stored in **target/release/rankit.exe**