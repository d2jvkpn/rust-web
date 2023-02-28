use sqlx::error::Error as SQLxError;

// TODO: sqlx::error::Error, sqlx::postgres::PgDatabaseError,
pub fn db_error_code(err: &SQLxError) -> Option<String> {
    let e2 = match err {
        sqlx::Error::Database(e) => e,
        _ => return None,
    };

    e2.code().map(|v| Some(v.into()))? // convert a Result to an option
}

pub fn pg_already_exists(err: &SQLxError) -> bool {
    match db_error_code(err) {
        None => false,
        Some(v) => v == "23505".to_string(),
    }
}

pub fn pg_not_found(err: &SQLxError) -> bool {
    match err {
        SQLxError::RowNotFound => true,
        _ => false,
    }
}
