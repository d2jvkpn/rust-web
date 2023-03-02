mod helpers;

use actix_web::http::StatusCode;
use helpers::spawn_app_create_db;

#[tokio::test]
async fn users() {
    let address = spawn_app_create_db().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/open/user/register", &address))
        .header("Content-Type", "application/json")
        .body(r#"{"email": "d2jvkpn@users.noreply.github.com", "name": "Rover", "birthday": "2006-01-02"}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let response = client
        .post(&format!("{}/api/open/user/register", &address))
        .header("Content-Type", "application/json")
        .body(r#"{"email": "d2jvkpn@users.noreply.github.com", "name": "Rover", "birthday": "2006-01-02"}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
}
