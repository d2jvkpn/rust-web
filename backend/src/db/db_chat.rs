use crate::{
    middlewares::{Error, QueryPage, QueryResult},
    models::chat::*,
};
// use chrono::Utc;
use sqlx::PgPool;

pub async fn new_chat_record(pool: &PgPool, item: ChatRecord) -> Option<Error> {
    match sqlx::query!(
        r#"INSERT INTO chats (request_id, user_id, query, query_at, response, response_at)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        item.request_id,
        item.user_id,
        item.query,
        item.query_at,
        item.response,
        item.response_at,
    )
    .execute(pool)
    .await
    {
        Ok(_) => None,
        Err(e) => Some(e.into()),
    }
}

pub async fn query_chat_records(
    pool: &PgPool,
    user_id: i32,
    page: QueryPage,
) -> Result<QueryResult<ChatRecord>, Error> {
    let mut result = QueryResult::new();

    result.items = sqlx::query_as!(
        ChatRecord,
        r#"SELECT request_id, user_id, query, query_at, response, response_at
        FROM chats WHERE user_id = $1 AND response IS NOT NULL
        ORDER BY query_at DESC LIMIT $2 OFFSET $3"#,
        user_id,
        page.page_size,
        page.page_size * (page.page_no - 1),
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
