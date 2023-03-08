mod helpers;

use actix_web::http::StatusCode;
use helpers::spawn_app_create_db;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ResData<T> {
    code: i32,
    msg: String,
    data: T,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Token {
    token_name: String,
    token_value: String,
}

#[tokio::test]
async fn users() {
    let address = spawn_app_create_db().await;
    let client = reqwest::Client::new();

    //
    let body = r#"{"email": "d2jvkpn@users.noreply.github.com", "name": "Rover",
    "birthday": "2006-01-02", "password": "12QWas!@"}"#;

    let response = client
        .post(&format!("{}/api/open/user/register", &address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let response = client
        .post(&format!("{}/api/open/user/register", &address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);

    //
    let response = client
        .post(&format!("{}/api/open/user/login", &address))
        .header("Content-Type", "application/json")
        .body(r#"{"email": "d2jvkpn@users.noreply.github.com", "password": "12QWas!@"}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let bts = response.bytes().await.unwrap();
    println!("--> {:?}", bts);
    let res_data: ResData<Token> = serde_json::from_slice(&bts).unwrap();
    let token = res_data.data;

    //
    let response = client
        .post(&format!("{}/api/auth/user/logout", &address))
        .header(&token.token_name, &token.token_value)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let res_data = client
        .post(&format!("{}/api/open/user/login", &address))
        .header("Content-Type", "application/json")
        .body(r#"{"email": "d2jvkpn@users.noreply.github.com", "password": "12QWas!@"}"#)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .expect("Failed to execute request.");

    dbg!(&res_data);
}
