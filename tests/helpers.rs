#![allow(dead_code)]

use log::LevelFilter;
use rust_web::{
    internal::{load_config, settings::Settings, startup::run_with_listener, Database},
    utils,
};
// use sqlx::{Connection, PgConnection};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

// connect to database in config
pub async fn spawn_app_without_db() -> String {
    let (listener, port) = utils::tcp_listener_with_random_port("127.0.0.1:0").unwrap();

    let config = load_config("configs/local.yaml").expect("Failed to read configuration");

    let pool = PgPool::connect(&config.database.to_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate the database");

    let server = run_with_listener(listener, pool).unwrap();
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

// connect to database in config
pub async fn spawn_app_create_db() -> String {
    let (listener, port) = utils::tcp_listener_with_random_port("127.0.0.1:0").unwrap();

    let config = load_config("configs/local.yaml").expect("Failed to read configuration");

    utils::init_logger(
        format!("logs/{}.test.log", env!("CARGO_PKG_NAME")).as_str(),
        LevelFilter::Debug,
        true,
    )
    .unwrap();

    let pool = prepare_test_db(&config.database).await;
    Settings::set(config, pool.clone()).unwrap();

    let server = run_with_listener(listener, pool).unwrap();
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

async fn prepare_test_db(dsn: &Database) -> PgPool {
    // remove temporary databases newsletter_test_*
    let mut conn =
        PgConnection::connect(&dsn.to_string()).await.expect("Failed to connect to Postgres");

    let test_dbs = sqlx::query!(
        r#"SELECT datname FROM pg_database
          WHERE datistemplate = false AND datname LIKE 'rust_web__test_%';"#
    )
    .fetch_all(&mut conn)
    .await
    .expect("Failed to fetch test databases.");

    for db in test_dbs {
        _ = conn.execute(format!(r#"DROP DATABASE "{}";"#, &db.datname).as_str()).await;
    }

    // create a temporary db
    let dbname = "rust_web__test_".to_owned() + &Uuid::new_v4().to_string();
    let test_db = dsn.conn.clone() + "/" + &dbname;

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, &dbname).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect(&test_db).await.expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate the database");

    pool
}
