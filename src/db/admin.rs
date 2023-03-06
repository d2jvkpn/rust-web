use crate::{
    middlewares::{response::Error, QueryPage, QueryResult},
    models::user::*,
    utils,
};
use bcrypt::DEFAULT_COST; // hash and verify are blocking tasks, use utils::bcrypt_hash and utils::bcrypt_verify instead
use sqlx::{PgPool, QueryBuilder};

pub const BCRYPT_COST: u32 = DEFAULT_COST;

#[allow(dead_code)]
pub async fn query_users_v1(
    pool: &PgPool,
    mut page: QueryPage,
) -> Result<QueryResult<User>, Error> {
    page.check(("id", &["id", "name", "email"]))
        .map_err(|e| Error::InvalidArgument(e.to_string()))?;

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
    page.check(("id", &["id", "name", "email"]))
        .map_err(|e| Error::InvalidArgument(e.to_string()))?;

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

pub async fn find_user(pool: &PgPool, match_user: MatchUser) -> Result<User, Error> {
    match_user.valid().map_err(|e| Error::InvalidArgument(e.to_string()))?;

    let mut query = QueryBuilder::new(r#"SELECT * FROM users WHERE "#);
    if let Some(v) = match_user.id {
        query.push("id = ");
        query.push_bind(v);
    } else if let Some(v) = match_user.phone {
        query.push("phone = ");
        query.push_bind(v);
    } else if let Some(v) = match_user.email {
        query.push("email = ");
        query.push_bind(v);
    }
    query.push(" LIMIT 1");

    let err = match query.build_query_as().fetch_one(pool).await {
        Ok(v) => return Ok(v),
        Err(e) => e,
    };

    if utils::pg_not_found(&err) {
        Err(Error::NotFound("user not found".into()))
    } else {
        Err(err.into())
    }
}

pub async fn update_user_role(pool: &PgPool, uus: UpdateUserRole) -> Result<(), Error> {
    let err = match sqlx::query!(
        "UPDATE users SET status = $1 WHERE id = $2 RETURNING id",
        uus.role as Role,
        uus.id,
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

pub async fn update_user_status(pool: &PgPool, uus: UpdateUserStatus) -> Result<(), Error> {
    let err = match sqlx::query!(
        "UPDATE users SET status = $1 WHERE id = $2 RETURNING id",
        uus.status as Status,
        uus.id,
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

pub async fn reset_user_password(pool: &PgPool, item: ResetPassword) -> Result<(), Error> {
    item.valid().map_err(|e| Error::InvalidArgument(e.into()))?;

    let password =
        utils::bcrypt_hash(item.new_password, BCRYPT_COST).await.map_err(|_| Error::Unknown)?;

    sqlx::query!(r#"UPDATE users SET password = $1 WHERE id = $2"#, password, item.user_id)
        .execute(pool)
        .await?;

    Ok(())
}
