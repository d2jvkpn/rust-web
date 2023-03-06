use crate::{middlewares::response, models::token::Token, utils::socket_addr_to_ip_network};
use sqlx::{types::ipnetwork::IpNetwork, PgPool};

pub async fn save_token(pool: &PgPool, token: Token) -> Result<(), response::Error> {
    let ip_addr: Option<IpNetwork> = match &token.ip {
        None => None,
        Some(v) => socket_addr_to_ip_network(v),
    };

    sqlx::query!(
        r#"INSERT INTO tokens
              (token_id, user_id, iat, exp, ip, platform, device, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, true)"#,
        token.token_id,
        token.user_id,
        token.iat,
        token.exp,
        ip_addr,
        token.platform as _,
        token.device,
    )
    .execute(pool)
    .await?;

    Ok(())
}
