use chrono::{Local, SecondsFormat};
use sqlx::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub started_at: String,
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let now = Local::now();

        Self {
            started_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
            health_check_response: "I'm good. You've already asked me".to_string(),
            visit_count: Mutex::new(0),
            pool,
        }
    }
}
