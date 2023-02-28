use crate::{
    handlers::response::Error,
    models::user::{CreateUser, Role, Status, User},
    utils,
};
use sqlx::{query_as, PgPool};

pub async fn post_new_user(pool: &PgPool, item: CreateUser) -> Result<User, Error> {
    item.valid().map_err(|e| Error::InvalidArgument(e.to_string()))?;

    // TODO: supporting enum convert between postgresql and rust in sqlx
    let err = match query_as!(
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
