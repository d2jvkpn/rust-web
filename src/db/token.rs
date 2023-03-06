use crate::{
    middlewares::response,
    models::token::{Platform, Token},
    models::user::Role,
    utils::socket_addr_to_ip_network,
};
use serde::{Deserialize, Serialize};
use sqlx::{types::ipnetwork::IpNetwork, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    // pub iss: String, // issuer
    // pub sub: String, // subject
    pub iat: i64, // issued at
    pub exp: i64, // expiry
    pub token_id: Uuid,
    pub user_id: i32,
    pub role: Role,
    pub platform: Platform,
}

impl From<JwtPayload> for Token {
    fn from(item: JwtPayload) -> Self {
        Self {
            token_id: item.token_id,
            user_id: item.user_id,
            iat: item.iat,
            exp: item.exp,
            ip: None,
            platform: item.platform,
            device: None,
            status: true,
        }
    }
}

impl Token {
    #[allow(dead_code)]
    pub async fn persist(&self, pool: &PgPool) -> Result<(), response::Error> {
        let ip_addr: Option<IpNetwork> = match &self.ip {
            None => None,
            Some(v) => socket_addr_to_ip_network(v),
        };

        sqlx::query!(
            r#"INSERT INTO tokens
              (token_id, user_id, iat, exp, ip, platform, device, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, true)"#,
            self.token_id,
            self.user_id,
            self.iat,
            self.exp,
            ip_addr,
            self.platform as _,
            self.device,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
