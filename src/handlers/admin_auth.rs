use crate::{
    db::user as db_user,
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

pub async fn update_user_status(
    app_state: web::Data<AppState>,
    uus: web::Query<UpdateUserStatus>,
) -> Result<HttpResponse, Error> {
    db_user::update_user_status(&app_state.pool, uus.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
    // TODO: disable token
}
