use crate::{
    handlers::response::Error,
    models::user::{CreateUser, Role, Status, UpdateUser, User},
    utils,
};
use sqlx::PgPool;

pub async fn post_new_user(pool: &PgPool, item: CreateUser) -> Result<User, Error> {
    item.valid().map_err(|e| Error::InvalidArgument(e.to_string()))?;

    // TODO: supporting enum convert between postgresql and rust in sqlx
    let err = match sqlx::query_as!(
        User,
        r#"INSERT INTO users
          (status, role, phone, email, name, birthday) VALUES ($1, $2, $3, $4, $5, $6)
		RETURNING
		  id, status AS "status: Status", role AS "role: Role",
		  phone, email, name, birthday, created_at, updated_at"#,
        Status::OK as Status,
        Role::Member as Role,
        item.phone,
        item.email,
        item.name,
        item.birthday,
    )
    .fetch_one(pool)
    .await
    {
        Ok(v) => return Ok(v),
        Err(e) => e,
    };

    if utils::db_error_code(&err) == Some("23505".into()) {
        Err(Error::AlreadyExists)
    } else {
        Err(Error::DBError(err))
    }
}

// Update course details
pub async fn update_user_details(
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

    sqlx::query!(
        "UPDATE users SET name = $1, birthday = $2 WHERE id = $3",
        user.name,
        user.birthday,
        user.id
    )
    .execute(pool)
    .await?;

    // WARNING: user.updated_at is unchange
    // ?? return part of user only
    Ok(user)
}
