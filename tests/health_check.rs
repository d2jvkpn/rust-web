mod helpers;

use helpers::spawn_app_v1;

#[tokio::test]
async fn health_check() {
    let address = spawn_app_v1().await;

    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/healthz", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}
