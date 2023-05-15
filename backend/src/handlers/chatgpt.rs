use crate::{
    db::db_chat,
    internal::settings::get_chatgpt,
    internal::AppState,
    middlewares::{Error, IntoResult, QueryPage},
    models::chat::{ChatRecord, Message},
    models::chatgpt::ChatCompletionsRequest,
    models::jwt::JwtPayload,
};
use actix_web::{
    error::Error as ActixError,
    web::{self, ReqData},
    HttpRequest, HttpResponse,
};
use chrono::Utc;
use uuid::Uuid;

pub async fn chat_completions(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    request_id: ReqData<Uuid>,
    jwt: ReqData<JwtPayload>,
    msg: web::Json<Message>,
) -> Result<HttpResponse, ActixError> {
    // Ok(HttpResponse::Ok(Message::new("HELLO".into())).json())

    let client = get_chatgpt().ok_or(Error::unexpected_error())?;
    let msg = msg.into_inner();

    let mut record = ChatRecord {
        request_id: *request_id,
        user_id: jwt.user_id,
        query: msg.content.clone(),
        query_at: Utc::now(),
        response: None,
        response_at: None,
    };

    let ccr: ChatCompletionsRequest = msg.into();

    // TODO: log error
    let res = client.chat_completions(&ccr).await.map_err(|e| Error::unavailable(e.into()))?;

    // TODO: ?? res.choices.len()
    record.response = Some(res.choices[0].message.content.clone());
    record.response_at = Some(Utc::now());

    // TODO: log error
    db_chat::new_chat_record(&app_state.pool, record).await;

    Ok(res).into_result(&mut request)
}

pub async fn query_chat_records(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
    page: web::Query<QueryPage>,
) -> Result<HttpResponse, ActixError> {
    db_chat::query_chat_records(&app_state.pool, jwt.user_id, page.into_inner())
        .await
        .into_result(&mut request)
}
