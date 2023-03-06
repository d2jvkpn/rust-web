// use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, str::FromStr};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Display, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "platform", rename_all = "snake_case")]
pub enum Platform {
    #[display(fmt = "web")]
    Web,
    Android,
    Ios,
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
pub struct Token {
    pub token_id: Uuid,
    pub user_id: i32,
    pub iat: i64,
    pub exp: i64,
    pub ip: Option<SocketAddr>,
    pub platform: Platform,
    pub device: Option<String>,
    pub status: bool,
    // pub updated_at: DateTime<Utc>,
}

#[cfg(tes)]
mod tests {
    use super::*;
    fn t_platform() {
        let s = format!("{}", Platform::Ios);
        assert_eq!(s.as_str(), "ios");
    }
}
