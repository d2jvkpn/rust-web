use super::db_token::disable_user_tokens;
use crate::{
    middlewares::{Error, QueryPage, QueryResult},
    models::user::*,
    utils,
};
use anyhow::anyhow;
use bcrypt::DEFAULT_COST; // hash and verify are blocking tasks, use utils::bcrypt_hash and utils::bcrypt_verify instead
use sqlx::{PgPool, QueryBuilder};

pub const BCRYPT_COST: u32 = DEFAULT_COST;

#[allow(dead_code)]
pub async fn query_users_v1(
    pool: &PgPool,
    mut page: QueryPage,
) -> Result<QueryResult<User>, Error> {
    page.check(("id", &["id", "name", "email"])).map_err(|s| Error::invalid().msg(s))?;

    let mut result = QueryResult::new();

    result.items = sqlx::query_as!(
        User,
        r#"SELECT id, status AS "status: Status", role AS "role: Role",
        phone, email, name, birthday, created_at, updated_at
        FROM users ORDER BY id ASC LIMIT $1 OFFSET $2"#,
        page.page_size,
        page.page_size * (page.page_no - 1),
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn query_users_v2(
    pool: &PgPool,
    mut page: QueryPage,
) -> Result<QueryResult<User>, Error> {
    page.check(("id", &["id", "name", "email"])).map_err(|s| Error::invalid().msg(s))?;

    // dbg!(&page);
    let mut result = QueryResult::new();

    if page.page_no == 1 {
        match sqlx::query!("SELECT COUNT(id) FROM users").fetch_one(pool).await {
            Ok(v) => result.total = v.count.unwrap_or(0),
            Err(e) => return Err(e.into()),
        };
    }

    // not need to using 'status AS "status: Status", role AS "role: Role"' with QueryBuilder, ??
    let mut query = QueryBuilder::new(r#"SELECT * FROM users ORDER BY "#);
    query.push(page.order_by); // using push_bind will make "order by" inefficiency
    query.push(if page.asc { " ASC" } else { " DESC" });

    query.push(" LIMIT ");
    query.push_bind(page.page_size);
    query.push(" OFFSET ");
    query.push_bind(page.page_size * (page.page_no - 1));

    //
    result.items = query.build_query_as().fetch_all(pool).await?;
    Ok(result)
}

pub async fn find_user(pool: &PgPool, item: MatchUser) -> Result<User, Error> {
    item.valid().map_err(|s| Error::invalid().msg(s))?;

    let mut query = QueryBuilder::new(r#"SELECT * FROM users WHERE "#);
    if let Some(v) = item.id {
        query.push("id = ");
        query.push_bind(v);
    } else if let Some(v) = item.phone {
        query.push("phone = ");
        query.push_bind(v);
    } else if let Some(v) = item.email {
        query.push("email = ");
        query.push_bind(v);
    }
    query.push(" LIMIT 1");

    let err = match query.build_query_as().fetch_one(pool).await {
        Ok(v) => return Ok(v),
        Err(e) => e,
    };

    if utils::pg_not_found(&err) {
        Err(Error::not_found().msg("user not found"))
    } else {
        Err(err.into())
    }
}

pub async fn update_user_role(pool: &PgPool, item: UpdateUserRole) -> Result<(), Error> {
    let err = match sqlx::query!(
        "UPDATE users SET status = $1 WHERE id = $2 RETURNING id",
        item.role as Role,
        item.user_id,
    )
    .fetch_one(pool)
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => e,
    };

    if utils::pg_not_found(&err) {
        Err(Error::not_found().msg("user not found"))
    } else {
        Err(err.into())
    }
}

pub async fn update_user_status(pool: &PgPool, item: UpdateUserStatus) -> Result<(), Error> {
    let status = item.status.clone();
    let result = sqlx::query!(
        "UPDATE users SET status = $1 WHERE id = $2 RETURNING id",
        item.status as Status,
        item.user_id,
    )
    .fetch_one(pool)
    .await;

    if let Err(e) = result {
        if utils::pg_not_found(&e) {
            return Err(Error::not_found().msg("user not found"));
        } else {
            return Err(e.into());
        }
    }

    if status != Status::OK {
        let _ = disable_user_tokens(pool, item.user_id, None).await;
    }

    Ok(())
}

pub async fn reset_user_password(pool: &PgPool, item: ResetPassword) -> Result<(), Error> {
    item.valid().map_err(|s| Error::invalid().msg(s))?;

    let password = utils::bcrypt_hash(item.new_password, BCRYPT_COST)
        .await
        .map_err(|s| Error::unknown().cause(anyhow!(s)))?;

    sqlx::query!(r#"UPDATE users SET password = $1 WHERE id = $2"#, password, item.user_id)
        .execute(pool)
        .await?;

    _ = disable_user_tokens(pool, item.user_id, None).await;

    Ok(())
}
