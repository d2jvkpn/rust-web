#![allow(dead_code)]

use rust_web::{internal::startup_v1::run_with_listener, utils};

// connect to database in config
pub async fn spawn_app_v1() -> String {
    let (listener, port) = utils::tcp_listener_with_random_port("127.0.0.1:0").unwrap();

    let server = run_with_listener(listener).unwrap();
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}
