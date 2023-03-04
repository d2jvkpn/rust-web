use crate::{
    db::user as db_user,
    internal::{settings::JwtPayload, AppState},
    middlewares::response::{Data, Error, OK_JSON},
    middlewares::QueryPage,
    models::user::*,
};
use actix_web::{
    http::header::ContentType,
    web::{self, ReqData},
    HttpResponse,
};

pub async fn update_user_details(
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_details_a(&app_state.pool, *user_id, item.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn update_user_details_v2a(
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_details_b(&app_state.pool, *user_id, item.into_inner()).await?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(OK_JSON))
}

pub async fn update_user_details_v2b(
    app_state: web::Data<AppState>,
    match_user: web::Query<MatchUser>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_details_b(&app_state.pool, match_user.id.unwrap_or(0), item.into_inner())
        .await?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(OK_JSON))
}

pub async fn update_user_details_v3(
    app_state: web::Data<AppState>,
    jwt_payload: ReqData<JwtPayload>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_details_b(&app_state.pool, jwt_payload.user_id, item.into_inner()).await?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(OK_JSON))
}

pub async fn query_users(
    app_state: web::Data<AppState>,
    query_page: web::Query<QueryPage>,
) -> Result<HttpResponse, Error> {
    db_user::query_users_v2(&app_state.pool, query_page.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn find_user(
    app_state: web::Data<AppState>,
    match_user: web::Query<MatchUser>,
) -> Result<HttpResponse, Error> {
    db_user::find_user(&app_state.pool, match_user.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn user_details(
    app_state: web::Data<AppState>,
    jwt_payload: ReqData<JwtPayload>,
) -> Result<HttpResponse, Error> {
    let match_user = MatchUser { id: Some(jwt_payload.user_id), ..Default::default() };
    db_user::find_user(&app_state.pool, match_user).await.map(|v| Ok(Data(v).into()))?
}

pub async fn update_user_status(
    app_state: web::Data<AppState>,
    uus: web::Query<UpdateUserStatus>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_status(&app_state.pool, uus.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}
