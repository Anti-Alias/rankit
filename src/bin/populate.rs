use reqwest::StatusCode;
use reqwest::multipart::{Form, Part};
use axum_test_helper::*;
use rankit::account::LoginRequest;
use rankit::app::read_var;
use rankit::{env, rank};
use rankit::{app, category, thing};
use serde_json::{to_vec, to_string};

/// Used to populate rankit api with some initial data.
/// Useful for testing purposes.
#[tokio::main]
async fn main() {

    dotenvy::dotenv().unwrap();
    env_logger::init();

    println!("___Starting APP___");
    let app = app::create_app_from_env(true).await.unwrap();
    let client = TestClient::new(app);
    let root_name: String = read_var(env::APP_ROOT_ACCOUNT_NAME).unwrap();
    let root_pass: String = read_var(env::APP_ROOT_ACCOUNT_PASSWORD).unwrap();

    println!("___Logging in as root___");
    let auth = login(&root_name, &root_pass, &client).await;

    println!("___Creating things___");
    let apple_bytes = include_bytes!("images/apple.jpg");
    let rice_bytes = include_bytes!("images/rice.jpg");
    let oatmeal_bytes = include_bytes!("images/oatmeal.jpg");
    let steak_bytes = include_bytes!("images/steak.jpg");
    let apple_id = create_thing("apple", apple_bytes, &auth, &client).await;
    let rice_id = create_thing("rice", rice_bytes, &auth, &client).await;
    let oatmeal_id = create_thing("oatmeal", oatmeal_bytes, &auth, &client).await;
    let steak_id = create_thing("steak", steak_bytes, &auth, &client).await;

    println!("___Creating categories___");
    let food_id = create_category("food", &auth, &client).await;
    let fruit_id = create_category("fruit", &auth, &client).await;
    let grain_id = create_category("grain", &auth, &client).await;
    let meat_id = create_category("meat", &auth, &client).await;

    println!("___Creating ranks__");
    create_rank(apple_id, food_id, &auth, &client).await;
    create_rank(apple_id, fruit_id, &auth, &client).await;
    create_rank(rice_id, food_id, &auth, &client).await;
    create_rank(rice_id, grain_id, &auth, &client).await;
    create_rank(oatmeal_id, food_id, &auth, &client).await;
    create_rank(oatmeal_id, grain_id, &auth, &client).await;
    create_rank(steak_id, food_id, &auth, &client).await;
    create_rank(steak_id, meat_id, &auth, &client).await;
}

async fn login(root_name: &str, root_pass: &str, client: &TestClient) -> String {
    let body = to_vec(&LoginRequest {
        name: Some(root_name.into()),
        email: None,
        password: root_pass.into()
    }).unwrap();
    let response = client.post("/account/login")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    expect_status(StatusCode::OK, response.status());
    let claims_str = response.text().await;
    let auth = format!("Bearer {claims_str}");
    println!("Auth: {auth}");
    auth
}

async fn create_thing(name: &str, bytes: &'static [u8], auth: &str, client: &TestClient) -> i32 {
    let thing_json = to_string(&thing::CreateRequest { name: name.into() }).unwrap();
    let form = Form::new()
        .text("request", thing_json)
        .part("image", Part::bytes(bytes));
    let response = client.post("/thing")
        .header("Authorization", auth)
        .multipart(form)
        .send()
        .await;
    expect_status(StatusCode::CREATED, response.status());
    let response_body: thing::CreateResponse = response.json().await;
    response_body.thing.id
}

async fn create_category(name: &str, auth: &str, client: &TestClient) -> i32 {
    let body = to_vec(&category::CreateRequest { name: name.into() }).unwrap();
    let response = client.post("/category")
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    expect_status(StatusCode::CREATED, response.status());
    let response_body: category::CreateResponse = response.json().await;
    response_body.category.id
}

async fn create_rank(thing_id: i32, category_id: i32, auth: &str, client: &TestClient) {
    let body = to_vec(&rank::CreateRequest { thing_id, category_id }).unwrap();
    let response = client.post("/rank")
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    expect_status(StatusCode::CREATED, response.status());
}

fn expect_status(expected: StatusCode, actual: StatusCode) {
    if actual != expected {
        panic!("Expected status {expected}. Got {actual}");
    }
    println!("Status {actual}");
}