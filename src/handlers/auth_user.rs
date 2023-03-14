use crate::{
    db::db_admin,
    db::db_token::disable_curent_token,
    db::db_user,
    internal::AppState,
    middlewares::{empty_data, IntoResult},
    models::{token::JwtPayload, user::*},
};
use actix_web::{
    error::Error as ActixError,
    web::{self, ReqData},
    HttpRequest, HttpResponse,
};

#[allow(dead_code)]
// POST /user/update/{user_id} + BODY, update_user_details_a
pub async fn update_user_details(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, ActixError> {
    db_user::update_user_details_a(&app_state.pool, *user_id, item.into_inner())
        .await
        .into_result(&mut request)
}

#[allow(dead_code)]
// POST /user/update/{user_id} + BODY, update_user_details_b
pub async fn update_user_details_v2a(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, ActixError> {
    db_user::update_user_details_b(&app_state.pool, *user_id, item.into_inner())
        .await
        .map(|_| empty_data())
        .into_result(&mut request)
}

#[allow(dead_code)]
// POST /user/update?user_id=1 + BODY, update_user_details_b
pub async fn update_user_details_v2b(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    match_user: web::Query<MatchUser>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, ActixError> {
    db_user::update_user_details_b(&app_state.pool, match_user.id.unwrap_or(0), item.into_inner())
        .await
        .map(|_| empty_data())
        .into_result(&mut request)
}

// POST /user/update + BODY, update_user_details_b
pub async fn update_user_details_v3(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, ActixError> {
    db_user::update_user_details_b(&app_state.pool, jwt.user_id, item.into_inner())
        .await
        .map(|_| empty_data())
        .into_result(&mut request)
}

pub async fn user_details(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
) -> Result<HttpResponse, ActixError> {
    let match_user = MatchUser { id: Some(jwt.user_id), ..Default::default() };
    db_admin::find_user(&app_state.pool, match_user).await.into_result(&mut request)
}

pub async fn frozen_user_status(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
) -> Result<HttpResponse, ActixError> {
    let item = UpdateUserStatus { user_id: jwt.user_id, status: Status::Frozen };

    db_admin::update_user_status(&app_state.pool, item).await.into_result(&mut request)
}

pub async fn user_change_password(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
    item: web::Json<ChangePassword>,
) -> Result<HttpResponse, ActixError> {
    db_user::user_change_password(&app_state.pool, jwt.user_id, item.into_inner())
        .await
        .into_result(&mut request)
}

pub async fn user_logout(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    jwt: ReqData<JwtPayload>,
) -> Result<HttpResponse, ActixError> {
    disable_curent_token(&app_state.pool, jwt.token_id).await.into_result(&mut request)
}
