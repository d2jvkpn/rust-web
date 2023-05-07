use crate::{
    middlewares::Error,
    models::jwt::{Platform, TokenRecord},
    utils::{self, socket_addr_to_ip_network},
};
use chrono::Utc;
use sqlx::{types::ipnetwork::IpNetwork, PgPool, QueryBuilder, Row};
use uuid::Uuid;

pub async fn save_token(pool: &PgPool, token_record: TokenRecord) -> Result<(), Error> {
    let ip_addr: Option<IpNetwork> = match &token_record.ip {
        None => None,
        Some(v) => socket_addr_to_ip_network(v),
    };

    sqlx::query!(
        r#"INSERT INTO tokens
          (token_id, user_id, iat, exp, ip, platform, device, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, true)"#,
        token_record.token_id,
        token_record.user_id,
        token_record.iat,
        token_record.exp,
        ip_addr,
        token_record.platform as _,
        token_record.device,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn disable_curent_token(pool: &PgPool, token_id: Uuid) -> Result<(), Error> {
    sqlx::query!(r#"UPDATE tokens SET status = false WHERE token_id = $1"#, token_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn disable_user_tokens(
    pool: &PgPool,
    user_id: i32,
    platform: Option<Platform>,
) -> Result<Vec<Uuid>, Error> {
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

pub async fn validate_token_in_table(pool: &PgPool, token_id: Uuid) -> Result<(), Error> {
    let err = match sqlx::query!(r#"SELECT status FROM tokens WHERE token_id = $1"#, token_id)
        .fetch_one(pool)
        .await
    {
        Ok(v) => {
            let err_msg = "token was disabled, relogin required";
            if v.status == Some(false) {
                return Err(Error::unauthenticated().msg(err_msg));
            }
            return Ok(());
        }
        Err(e) => e,
    };

    if utils::pg_not_found(&err) {
        Err(Error::unauthenticated().msg("can't verify token, relogin required"))
    } else {
        Err(Error::db_error(err))
    }
}
