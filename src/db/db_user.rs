use super::{
    db_admin::BCRYPT_COST,
    db_token::{disable_user_tokens, save_token},
};
use crate::{
    internal::settings::Settings,
    middlewares::response::Error,
    models::{
        token::{JwtPayload, Platform, Token},
        user::*,
    },
    utils,
};
use sqlx::{PgPool, QueryBuilder};
use std::{net::SocketAddr, time::Duration};
use uuid::Uuid;

pub async fn post_new_user(pool: &PgPool, item: CreateUser) -> Result<User, Error> {
    item.valid().map_err(|e| Error::InvalidArgument(e.to_string()))?;

    let password =
        utils::bcrypt_hash(item.password, BCRYPT_COST).await.map_err(|_| Error::Unknown)?;
    // dbg!(&password);

    // TODO: supporting enum convert between postgresql and rust in sqlx
    // https://docs.rs/sqlx/latest/sqlx/macro.query.html
    let err = match sqlx::query_as!(
        User,
        r#"INSERT INTO users
          (status, role, phone, email, name, birthday, password)
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
    {
        Ok(v) => return Ok(v),
        Err(e) => e,
    };

    dbg!(&err);
    if utils::pg_already_exists(&err) {
        Err(Error::AlreadyExists)
    } else {
        Err(Error::DBError(err))
    }
}

// Update course details, select, compare, update
pub async fn update_user_details_a(
    pool: &PgPool,
    user_id: i32,
    update_user: UpdateUser,
) -> Result<User, Error> {
    if user_id <= 0 {
        return Err(Error::InvalidArgument("invalid user_id".into()));
    }
    if let Err(e) = update_user.valid() {
        return Err(Error::InvalidArgument(e.to_string()));
    }

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
    .map_err(|_err| Error::NotFound("user not found".into()))?;
    // ignore database errors other than NotFound

    if !user.update(update_user) {
        return Err(Error::NoChanges);
    }

    let err = match sqlx::query!(
        "UPDATE users SET name = $1, birthday = $2 WHERE id = $3",
        user.name,
        user.birthday,
        user.id
    )
    .execute(pool)
    .await
    {
        Ok(_) => return Ok(user),
        Err(e) => e,
    };

    // WARNING: user.updated_at is unchange
    // ?? return part of user only
    if utils::pg_not_found(&err) {
        Err(Error::NotFound("user not found".into()))
    } else {
        Err(err.into())
    }
}

// Update course details, update and return id
pub async fn update_user_details_b(
    pool: &PgPool,
    user_id: i32,
    update_user: UpdateUser,
) -> Result<(), Error> {
    if user_id <= 0 {
        return Err(Error::InvalidArgument("invalid user_id".into()));
    }
    if let Err(e) = update_user.valid() {
        return Err(Error::InvalidArgument(e.to_string()));
    }

    let err = match sqlx::query!(
        "UPDATE users SET name = $1, birthday = $2 WHERE id = $3 RETURNING id",
        update_user.name,
        update_user.birthday,
        user_id,
    )
    .fetch_one(pool)
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => e,
    };

    if utils::pg_not_found(&err) {
        Err(Error::NotFound("user not found".into()))
    } else {
        Err(err.into())
    }
}

pub async fn user_login(
    pool: &PgPool,
    login: UserLogin,
    ip: Option<SocketAddr>,
    platform: Platform,
) -> Result<UserAndToken, Error> {
    login.valid().map_err(|e| Error::InvalidArgument(e.into()))?;

    // let now = std::time::Instant::now(); println!("--> user_login offset: {:?}", now.elapsed());
    let mut query = QueryBuilder::new(r#"SELECT * FROM users WHERE "#);
    if let Some(v) = login.phone {
        query.push("phone = ");
        query.push_bind(v);
    } else if let Some(v) = login.email {
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
                return Err(Error::NotFound(err_msg));
            } else {
                return Err(e.into());
            };
        }
    };

    upassword.user.status_ok().map_err(|e| Error::PermissionDenied(e.into()))?;

    let m = utils::bcrypt_verify(login.password, upassword.password)
        .await
        .map_err(|_| Error::Unknown)?;
    if !m {
        return Err(Error::NotFound(err_msg));
    }

    let mut playload = JwtPayload {
        iat: 0,
        exp: 0,
        token_id: Uuid::new_v4(),
        user_id: upassword.user.id,
        role: upassword.user.role.clone(),
        platform: platform.clone(),
    };
    let token_value = Settings::jwt_sign(&mut playload)?;

    // TODO: use a message queue or a channel instead
    let mut token_record: Token = playload.into();
    (token_record.ip, token_record.device) = (ip, None);
    disable_user_tokens(pool, upassword.user.id, Some(platform)).await?;
    save_token(pool, token_record).await?;

    Ok(UserAndToken { user: upassword.user, token_name: "authorization".to_string(), token_value })
}

pub async fn user_change_password(
    pool: &PgPool,
    user_id: i32,
    item: ChangePassword,
) -> Result<(), Error> {
    item.valid().map_err(|e| Error::InvalidArgument(e.into()))?;

    let mut query = QueryBuilder::new(r#"SELECT * FROM users WHERE "#);
    query.push("id = ");
    query.push_bind(user_id);

    let err_msg = "user not found or incorrect password".to_string();
    let upassword: UserAndPassword = match query.build_query_as().fetch_one(pool).await {
        Ok(v) => v,
        Err(e) => {
            tokio::time::sleep(Duration::from_secs(1)).await;
            if utils::pg_not_found(&e) {
                return Err(Error::NotFound(err_msg));
            } else {
                return Err(e.into());
            };
        }
    };

    upassword.user.status_ok().map_err(|e| Error::PermissionDenied(e.into()))?;
    let m = utils::bcrypt_verify(item.old_password, upassword.password)
        .await
        .map_err(|_| Error::Unknown)?;

    if !m {
        return Err(Error::NotFound(err_msg));
    }

    let password =
        utils::bcrypt_hash(item.new_password, BCRYPT_COST).await.map_err(|_| Error::Unknown)?;

    sqlx::query!(r#"UPDATE users SET password = $1 WHERE id = $2"#, password, user_id)
        .execute(pool)
        .await?;

    Ok(())
}
