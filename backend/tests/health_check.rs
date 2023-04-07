mod helpers;

use actix_web::http::StatusCode;
use helpers::spawn_app_without_db;

#[tokio::test]
async fn health_check() {
    let address = spawn_app_without_db().await;

    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/healthz", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn not_exists() {
    let address = spawn_app_without_db().await;

    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/not_exists", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let content_type = response.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(content_type.starts_with("application/json"));
}
