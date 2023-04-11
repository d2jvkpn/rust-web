use super::{
    db_admin::BCRYPT_COST,
    db_token::{disable_curent_token, disable_user_tokens, save_token, validate_token_in_table},
};
use crate::{
    internal::settings::Settings,
    middlewares::Error,
    models::token::{JwtPayload, Platform, TokenKind, TokenRecord},
    models::user::*,
    utils,
};
use anyhow::anyhow;
// use chrono::Utc;
use sqlx::{PgPool, QueryBuilder};
use std::{net::SocketAddr, time::Duration};
use uuid::Uuid;

pub async fn post_new_user(pool: &PgPool, item: CreateUser) -> Result<User, Error> {
    item.valid().map_err(|s| Error::invalid().msg(s))?;

    let password = utils::bcrypt_hash(item.password, BCRYPT_COST)
        .await
        .map_err(|s| Error::unknown().cause(anyhow!(s)))?;
    // dbg!(&password);

    // TODO: supporting enum convert between postgresql and rust in sqlx
    // https://docs.rs/sqlx/latest/sqlx/macro.query.html
    sqlx::query_as!(
        User,
        r#"INSERT INTO users (status, role, phone, email, name, birthday, password)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
          id, status AS "status: Status", role AS "role: Role",
          phone, email, name, birthday, created_at, updated_at"#,
        Status::OK as Status,
        Role::Member as Role,
        item.phone,
        item.email,
        item.name,
        item.birthday,
        password,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::db_check_already_exists(e, "user"))
}

// Update course details, select, compare, update
pub async fn update_user_details_a(
    pool: &PgPool,
    user_id: i32,
    item: UpdateUser,
) -> Result<User, Error> {
    if user_id <= 0 {
        return Err(Error::invalid().msg("invalid user_id"));
    }
    item.valid().map_err(|s| Error::invalid().msg(s))?;

    // Retrieve current record
    let mut user = sqlx::query_as!(
        User,
        r#"SELECT id, status AS "status: Status", role AS "role: Role",
          phone, email, name, birthday, created_at, updated_at
        FROM users WHERE id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::db_check_not_found(e, "user"))?;

    if !user.update(item) {
        return Err(Error::no_changes());
    }

    sqlx::query!(
        "UPDATE users SET name = $1, birthday = $2 WHERE id = $3",
        user.name,
        user.birthday,
        user.id
    )
    .execute(pool)
    .await
    .map_err(|e| Error::db_check_not_found(e, "user"))?;
    // WARNING: user.updated_at is unchange
    // ?? return part of user only

    Ok(user)
}

// Update course details, update and return id
pub async fn update_user_details_b(
    pool: &PgPool,
    user_id: i32,
    item: UpdateUser,
) -> Result<(), Error> {
    if user_id <= 0 {
        return Err(Error::invalid().msg("invalid user_id"));
    }
    item.valid().map_err(|s| Error::invalid().msg(s))?;

    sqlx::query!(
        "UPDATE users SET name = $1, birthday = $2 WHERE id = $3 RETURNING id",
        item.name,
        item.birthday,
        user_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::db_check_not_found(e, "user"))?;

    Ok(())
}

pub async fn user_login(
    pool: &PgPool,
    item: UserLogin,
    ip: Option<SocketAddr>,
    platform: Platform,
    request_id: Uuid,
) -> Result<UserAndTokens, Error> {
    item.valid().map_err(|s| Error::invalid().msg(s))?;

    // step1
    // let now = std::time::Instant::now(); println!("--> user_login offset: {:?}", now.elapsed());
    let mut query = QueryBuilder::new(r#"SELECT * FROM users WHERE "#);
    if let Some(v) = item.phone {
        query.push("phone = ");
        query.push_bind(v);
    } else if let Some(v) = item.email {
        query.push("email = ");
        query.push_bind(v);
    }
    query.push(" LIMIT 1");

    let err_msg = "user not found or incorrect password".to_string();
    let upassword: UserAndPassword = match query.build_query_as().fetch_one(pool).await {
        Ok(v) => v,
        Err(e) => {
            if utils::pg_not_found(&e) {
                tokio::time::sleep(Duration::from_secs(1)).await;
                return Err(Error::unauthenticated().msg(err_msg));
            } else {
                return Err(e.into());
            };
        }
    };

    upassword.user.status_ok().map_err(|s| Error::permission_denied().msg(s))?;

    // step2
    let m = utils::bcrypt_verify(item.password, upassword.password)
        .await
        .map_err(|s| Error::unknown().cause(anyhow!(s)))?;
    if !m {
        return Err(Error::not_found().msg(err_msg));
    }

    // step3
    let mut playload = JwtPayload {
        iat: 0,
        exp: 0,
        token_id: request_id,
        token_kind: TokenKind::Access,
        user_id: upassword.user.id,
        role: upassword.user.role.clone(),
        platform: platform.clone(),
    };
    let tokens = Settings::jwt_sign(&mut playload)?;

    let mut token_record: TokenRecord = playload.into();
    (token_record.ip, token_record.device) = (ip, None); // TODO: device

    // step4
    disable_user_tokens(pool, upassword.user.id, Some(platform)).await?;
    save_token(pool, token_record).await?;

    Ok(UserAndTokens { user: upassword.user, tokens })
}

pub async fn user_change_password(
    pool: &PgPool,
    user_id: i32,
    item: ChangePassword,
) -> Result<(), Error> {
    item.valid().map_err(|s| Error::invalid().msg(s))?;

    let mut query = QueryBuilder::new(r#"SELECT * FROM users WHERE "#);
    query.push("id = ");
    query.push_bind(user_id);

    let err_msg = "user not found or incorrect password";
    let upassword: UserAndPassword = query
        .build_query_as()
        .fetch_one(pool)
        .await
        .map_err(|e| Error::db_check_not_found(e, err_msg))?;

    upassword.user.status_ok().map_err(|s| Error::permission_denied().msg(s))?;
    let m = utils::bcrypt_verify(item.old_password, upassword.password)
        .await
        .map_err(|s| Error::unknown().cause(anyhow!(s)))?;

    if !m {
        return Err(Error::not_found().msg(err_msg));
    }

    let password = utils::bcrypt_hash(item.new_password, BCRYPT_COST)
        .await
        .map_err(|s| Error::unknown().cause(anyhow!(s)))?;

    sqlx::query!(r#"UPDATE users SET password = $1 WHERE id = $2"#, password, user_id)
        .execute(pool)
        .await?;

    disable_user_tokens(pool, user_id, None).await?;

    Ok(())
}

pub async fn refresh_token(
    pool: &PgPool,
    item: RefreshToken,
    ip: Option<SocketAddr>,
    platform: Platform,
    request_id: Uuid,
) -> Result<Tokens, Error> {
    // step1
    let data = Settings::jwt_verify_token(&item.refresh_token, TokenKind::Refresh)?;

    /*
    if data.exp - Utc::now().timestamp() < 15 {
        return Err(Error::bad_request());
    }
    */

    // step2
    validate_token_in_table(pool, data.token_id).await?;

    // step3
    let mut query = QueryBuilder::new(r#"SELECT * FROM users WHERE "#);
    query.push("id = ");
    query.push_bind(data.user_id);

    let user: User = query
        .build_query_as()
        .fetch_one(pool)
        .await
        .map_err(|e| Error::db_check_not_found(e, "user"))?;

    user.status_ok().map_err(|s| Error::permission_denied().msg(s))?;

    // step4
    let mut playload = JwtPayload {
        iat: 0,
        exp: data.exp,
        token_id: request_id,
        token_kind: TokenKind::Refresh,
        user_id: user.id,
        role: user.role.clone(),
        platform: platform.clone(),
    };
    let tokens = Settings::jwt_sign(&mut playload)?;

    let mut token_record: TokenRecord = playload.into();
    (token_record.ip, token_record.device) = (ip, None); // TODO: device

    // step5
    disable_curent_token(pool, data.token_id).await?;
    save_token(pool, token_record).await?;

    Ok(tokens)
}
