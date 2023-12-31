use std::time::SystemTime;
use base64::{Engine as _, engine::general_purpose};
use rankit::account::RoleLesser;
use reqwest::StatusCode;
use reqwest::multipart::{Form, Part};
use axum_test_helper::*;
use rankit::app::read_var;
use rankit::{env_names, rank};
use rankit::{app, category, thing, account};
use serde_json::{to_vec, to_string};
use tokio::join;

/// Used to populate rankit api with some initial data.
/// Useful for testing purposes.
#[tokio::main]
async fn main() {

    dotenvy::dotenv().unwrap();
    env_logger::init();
    let start_time = SystemTime::now();

    println!("___Starting API___");
    let app = app::create_app_from_env(true).await.unwrap();
    let client = TestClient::new(app);
    let root_email: String = read_var(env_names::API_ROOT_ACCOUNT_EMAIL).unwrap();
    let root_pass: String = read_var(env_names::API_ROOT_ACCOUNT_PASSWORD).unwrap();

    println!("___Logging in as root___");
    let root_bearer = login(&root_email, &root_pass, &client).await;

    println!("___Creating accounts___");
    let(_basic_id, admin_id) = join!(
        create_account("basic", "basic@gmail.com", "password", &client),
        create_account("admin", "admin@gmail.com", "password", &client),
    );

    println!("___Setting roles___");
    set_role(admin_id, RoleLesser::Admin, &root_bearer, &client).await;

    println!("___Creating things___");
    let (apple_id, rice_id, oatmeal_id, steak_id, chicken_id, pork_id) = join!(
        create_thing("apple",   include_bytes!("images/apple.jpg"),     &root_bearer, &client),
        create_thing("rice",    include_bytes!("images/rice.jpg"),      &root_bearer, &client),
        create_thing("oatmeal", include_bytes!("images/oatmeal.jpg"),   &root_bearer, &client),
        create_thing("steak",   include_bytes!("images/steak.jpg"),     &root_bearer, &client),
        create_thing("chicken", include_bytes!("images/chicken.jpg"),   &root_bearer, &client),
        create_thing("pork",    include_bytes!("images/pork.jpg"),      &root_bearer, &client),
    );

    println!("___Creating categories___");
    let (food_id, fruit_id, grain_id, meat_id) = join!(
        create_category("food",     &root_bearer, &client),
        create_category("fruit",    &root_bearer, &client),
        create_category("grain",    &root_bearer, &client),
        create_category("meat",     &root_bearer, &client),
    );

    println!("___Creating ranks__");
    join!(
        create_rank(apple_id,   food_id,    &root_bearer, &client),
        create_rank(apple_id,   fruit_id,   &root_bearer, &client),
        create_rank(rice_id,    food_id,    &root_bearer, &client),
        create_rank(rice_id,    grain_id,   &root_bearer, &client),
        create_rank(oatmeal_id, food_id,    &root_bearer, &client),
        create_rank(oatmeal_id, grain_id,   &root_bearer, &client),
        create_rank(steak_id,   food_id,    &root_bearer, &client),
        create_rank(steak_id,   meat_id,    &root_bearer, &client),
        create_rank(chicken_id, food_id,    &root_bearer, &client),
        create_rank(chicken_id, meat_id,    &root_bearer, &client),
        create_rank(pork_id,    food_id,    &root_bearer, &client),
        create_rank(pork_id,    meat_id,    &root_bearer, &client),
    );

    let duration = SystemTime::now().duration_since(start_time).unwrap();
    println!("Duration: {:.2} seconds", duration.as_secs_f32());
}

async fn login(root_email: &str, root_pass: &str, client: &TestClient) -> String {
    let credentials = format!("{}:{}", root_email, root_pass);
    let credentials = general_purpose::STANDARD_NO_PAD.encode(credentials);
    let response = client.post("/api/account/login")
        .header("Authorization", format!("Basic {}", credentials))
        .send()
        .await;
    expect_status(StatusCode::OK, response.status());
    let claims_str = response.text().await;
    let bearer = format!("Bearer {claims_str}");
    println!("{bearer}");
    bearer
}

async fn create_account(name: &str, email: &str, password: &str, client: &TestClient) -> i32 {
    let body = to_string(&account::CreateRequest { name: name.into(), email: email.into(), password: password.into() }).unwrap();
    let response = client.post("/api/account")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    expect_status(StatusCode::CREATED, response.status());
    let response_body: account::CreateResponse = response.json().await;
    response_body.id
}

async fn set_role(account_id: i32, role: RoleLesser, bearer: &str, client: &TestClient) {
    let body = to_string(&account::UpdateRoleRequest { account_id, role }).unwrap();
    let response = client.put("/api/account/role")
        .header("Authorization", bearer)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    expect_status(StatusCode::NO_CONTENT, response.status());
}

async fn create_thing(name: &str, bytes: &'static [u8], bearer: &str, client: &TestClient) -> i32 {
    let thing_json = to_string(&thing::CreateRequest { name: name.into() }).unwrap();
    let form = Form::new()
        .text("request", thing_json)
        .part("image", Part::bytes(bytes));
    let response = client.post("/api/thing")
        .header("Authorization", bearer)
        .multipart(form)
        .send()
        .await;
    expect_status(StatusCode::CREATED, response.status());
    let response_body: thing::CreateResponse = response.json().await;
    response_body.thing.id
}

async fn create_category(name: &str, bearer: &str, client: &TestClient) -> i32 {
    let body = to_vec(&category::CreateRequest { name: name.into() }).unwrap();
    let response = client.post("/api/category")
        .header("Authorization", bearer)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    expect_status(StatusCode::CREATED, response.status());
    let response_body: category::CreateResponse = response.json().await;
    response_body.category.id
}

async fn create_rank(thing_id: i32, category_id: i32, bearer: &str, client: &TestClient) {
    let body = to_vec(&rank::CreateRequest { thing_id, category_id }).unwrap();
    let response = client.post("/api/rank")
        .header("Authorization", bearer)
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
}