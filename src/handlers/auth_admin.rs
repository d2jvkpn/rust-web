use crate::{
    db::admin as db_admin,
    db::token::disable_user_tokens,
    internal::AppState,
    middlewares::response::{Data, Error},
    middlewares::QueryPage,
    models::user::*,
};
use actix_web::{web, HttpResponse};

pub async fn query_users(
    app_state: web::Data<AppState>,
    query_page: web::Query<QueryPage>,
) -> Result<HttpResponse, Error> {
    db_admin::query_users_v2(&app_state.pool, query_page.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn find_user(
    app_state: web::Data<AppState>,
    match_user: web::Query<MatchUser>,
) -> Result<HttpResponse, Error> {
    db_admin::find_user(&app_state.pool, match_user.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn update_user_status(
    app_state: web::Data<AppState>,
    uus: web::Query<UpdateUserStatus>,
) -> Result<HttpResponse, Error> {
    if uus.status != Status::OK {
        let _ = disable_user_tokens(&app_state.pool, uus.user_id, None).await;
    }

    db_admin::update_user_status(&app_state.pool, uus.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn update_user_role(
    app_state: web::Data<AppState>,
    uur: web::Query<UpdateUserRole>,
) -> Result<HttpResponse, Error> {
    let _ = disable_user_tokens(&app_state.pool, uur.user_id, None).await;

    db_admin::update_user_role(&app_state.pool, uur.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn reset_user_password(
    app_state: web::Data<AppState>,
    reset_password: web::Json<ResetPassword>,
) -> Result<HttpResponse, Error> {
    let _ = disable_user_tokens(&app_state.pool, reset_password.user_id, None).await;

    db_admin::reset_user_password(&app_state.pool, reset_password.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}
