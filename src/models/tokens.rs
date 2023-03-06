// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "platform", rename_all = "snake_case")]
pub enum Platform {
    Web,
    Android,
    Ios,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub token_id: Uuid,
    pub user_id: i32,
    pub iat: i32,
    pub exp: i32,
    pub ip: String,
    pub platform: Platform,
    pub device: Option<String>,
    pub status: bool,
    // pub updated_at: DateTime<Utc>,
}
