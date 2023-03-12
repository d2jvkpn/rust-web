use crate::{
    db::db_admin,
    db::db_token::disable_user_tokens,
    internal::AppState,
    middlewares::{IntoResult, QueryPage},
    models::user::*,
};
use actix_web::{error::Error as ActixError, web, HttpRequest, HttpResponse};

pub async fn query_users(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    page: web::Query<QueryPage>,
) -> Result<HttpResponse, ActixError> {
    db_admin::query_users_v2(&app_state.pool, page.into_inner()).await.into_result(&mut request)
}

pub async fn find_user(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    item: web::Query<MatchUser>,
) -> Result<HttpResponse, ActixError> {
    db_admin::find_user(&app_state.pool, item.into_inner()).await.into_result(&mut request)
}

pub async fn update_user_status(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    item: web::Query<UpdateUserStatus>,
) -> Result<HttpResponse, ActixError> {
    if item.status != Status::OK {
        let _ = disable_user_tokens(&app_state.pool, item.user_id, None).await;
    }

    db_admin::update_user_status(&app_state.pool, item.into_inner()).await.into_result(&mut request)
}

pub async fn update_user_role(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    item: web::Query<UpdateUserRole>,
) -> Result<HttpResponse, ActixError> {
    let _ = disable_user_tokens(&app_state.pool, item.user_id, None).await;

    db_admin::update_user_role(&app_state.pool, item.into_inner()).await.into_result(&mut request)
}

pub async fn reset_user_password(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    item: web::Json<ResetPassword>,
) -> Result<HttpResponse, ActixError> {
    let _ = disable_user_tokens(&app_state.pool, item.user_id, None).await;

    db_admin::reset_user_password(&app_state.pool, item.into_inner())
        .await
        .into_result(&mut request)
}
