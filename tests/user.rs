mod helpers;

use actix_web::http::StatusCode;
use helpers::spawn_app_create_db;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response<T> {
    code: i32,
    msg: String,
    data: T,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Tokens {
    access_token: String,
    access_exp: i64,
    refresh_token: String,
    refresh_exp: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    tokens: Tokens,
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
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);

    //
    let response = client
        .post(&format!("{}/api/open/user/login", &address))
        .header("Content-Type", "application/json")
        .body(r#"{"email": "d2jvkpn@users.noreply.github.com", "password": "12QWas!@"}"#)
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let bts = response.bytes().await.unwrap();
    println!("--> {:?}", bts);
    let res: Response<LoginResponse> = serde_json::from_slice(&bts).unwrap();
    let access_token = "Bearer ".to_owned() + &res.data.tokens.access_token;

    let response = client
        .post(&format!("{}/api/auth/user/logout", &address))
        .header("authorization", &access_token)
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let res_json = client
        .post(&format!("{}/api/open/user/login", &address))
        .header("Content-Type", "application/json")
        .body(r#"{"email": "d2jvkpn@users.noreply.github.com", "password": "12QWas!@"}"#)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    dbg!(&res_json);

    //
    let data = client
        .post(&format!("{}/api/open/user/refresh_token", &address))
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"refreshToken": "{}"}}"#, &res.data.tokens.refresh_token))
        .send()
        .await
        .unwrap()
        .json::<Response<Tokens>>()
        .await
        .unwrap();

    println!("--> {:?}", data);
}
