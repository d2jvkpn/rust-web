use crate::{
    middlewares::response,
    models::token::{Platform, Token},
    utils::socket_addr_to_ip_network,
};
use chrono::Utc;
use sqlx::{error::Error as SQLxError, types::ipnetwork::IpNetwork, PgPool, QueryBuilder, Row};
use uuid::Uuid;

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

// TODO: use a message queue or in memory cache instead
pub async fn disable_curent_token(pool: &PgPool, token_id: Uuid) {
    let _ = sqlx::query!(r#"UPDATE tokens SET status = false WHERE token_id = $1"#, token_id)
        .execute(pool)
        .await;
}

pub async fn disable_user_tokens(
    pool: &PgPool,
    user_id: i32,
    platform: Option<Platform>,
) -> Result<Vec<Uuid>, SQLxError> {
    let exp = Utc::now().timestamp();
    /*
    let token_ids: Vec<Uuid> = match sqlx::query!(
        r#"UPDATE tokens SET status = false
        WHERE user_id = $1 AND exp > $2 AND status
        RETURNING token_id"#,
        user_id,
        exp,
    )
    .fetch_all(pool)
    .await
    {
        Err(e) => return Err(e),
        Ok(items) => items.into_iter().map(|v| v.token_id).collect(),
    };
    */

    let mut query = QueryBuilder::new(r#"UPDATE tokens SET status = false WHERE user_id = "#);
    query.push_bind(user_id);
    query.push(" AND exp > ");
    query.push_bind(exp);
    query.push(" AND status");
    if let Some(v) = platform {
        query.push(" AND platform = ");
        query.push_bind(v);
    }

    query.push(" RETURNING token_id");

    let token_ids: Vec<Uuid> =
        query.build().fetch_all(pool).await?.into_iter().map(|v| v.get(0)).collect();

    Ok(token_ids)
}
