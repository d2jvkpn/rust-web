// use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Display, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "platform", rename_all = "snake_case")]
pub enum Platform {
    #[display(fmt = "web")]
    Web,
    #[display(fmt = "android")]
    Android,
    #[display(fmt = "ios")]
    Ios,
    #[display(fmt = "unknown")]
    Unknown,
}

impl FromStr for Platform {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let val = match value {
            "web" => Self::Web,
            "android" => Self::Android,
            "ios" => Self::Ios,
            _ => return Err("unknown platform"),
        };

        Ok(val)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub token_id: Uuid,
    pub user_id: i32,
    pub iat: i32,
    pub exp: i32,
    pub ip: Option<SocketAddr>,
    pub platform: Platform,
    pub device: Option<String>,
    pub status: bool,
    // pub updated_at: DateTime<Utc>,
}
