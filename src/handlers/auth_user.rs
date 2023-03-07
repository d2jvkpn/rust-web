use crate::{
    db::admin as db_admin,
    db::token::disable_curent_token,
    db::user as db_user,
    internal::AppState,
    middlewares::response::{Data, Error, OK_JSON},
    models::{token::JwtPayload, user::*},
};
use actix_web::{
    http::header::ContentType,
    web::{self, ReqData},
    HttpResponse,
};

#[allow(dead_code)]
// POST /user/update/{user_id} + BODY, update_user_details_a
pub async fn update_user_details(
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_details_a(&app_state.pool, *user_id, item.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

#[allow(dead_code)]
// POST /user/update/{user_id} + BODY, update_user_details_b
pub async fn update_user_details_v2a(
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_details_b(&app_state.pool, *user_id, item.into_inner()).await?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(OK_JSON))
}

#[allow(dead_code)]
// POST /user/update?user_id=1 + BODY, update_user_details_b
pub async fn update_user_details_v2b(
    app_state: web::Data<AppState>,
    match_user: web::Query<MatchUser>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_details_b(&app_state.pool, match_user.id.unwrap_or(0), item.into_inner())
        .await?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(OK_JSON))
}

// POST /user/update + BODY, update_user_details_b
pub async fn update_user_details_v3(
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_details_b(&app_state.pool, jwt.user_id, item.into_inner()).await?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(OK_JSON))
}

pub async fn user_details(
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
) -> Result<HttpResponse, Error> {
    let match_user = MatchUser { id: Some(jwt.user_id), ..Default::default() };
    db_admin::find_user(&app_state.pool, match_user).await.map(|v| Ok(Data(v).into()))?
}

pub async fn frozen_user_status(
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
) -> Result<HttpResponse, Error> {
    let uus = UpdateUserStatus { user_id: jwt.user_id, status: Status::Frozen };

    disable_curent_token(&app_state.pool, jwt.token_id).await?;

    db_admin::update_user_status(&app_state.pool, uus).await.map(|v| Ok(Data(v).into()))?
}

pub async fn user_change_password(
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
    item: web::Json<ChangePassword>,
) -> Result<HttpResponse, Error> {
    disable_curent_token(&app_state.pool, jwt.token_id).await?;

    db_user::user_change_password(&app_state.pool, jwt.user_id, item.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn user_logout(
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
) -> Result<HttpResponse, Error> {
    disable_curent_token(&app_state.pool, jwt.token_id).await.map(|v| Ok(Data(v).into()))?
}
