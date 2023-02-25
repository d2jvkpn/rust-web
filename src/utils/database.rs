use sqlx::error::Error as SQLxError;

// TODO: sqlx::error::Error, sqlx::postgres::PgDatabaseError,
pub fn db_error_code(err: &SQLxError) -> Option<String> {
    let e2 = match err {
        sqlx::Error::Database(e) => e,
        _ => return None,
    };

    e2.code().map(|v| Some(v.into()))? // convert a Result to an option
}
