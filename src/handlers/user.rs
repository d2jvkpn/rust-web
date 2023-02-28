use crate::{
    db,
    handlers::response::{Data, Error, OK_JSON},
    internal::AppState,
    models::user::{CreateUser, QueryUser, UpdateUser},
};
use actix_web::{http::header::ContentType, web, HttpResponse};

pub async fn post_new_user(
    app_state: web::Data<AppState>,
    item: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    db::user::post_new_user(&app_state.pool, item.into_inner()).await.map(|v| Ok(Data(v).into()))?
}

pub async fn update_user_details(
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db::user::update_user_details(&app_state.pool, *user_id, item.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn update_user_details_v2a(
    app_state: web::Data<AppState>,
    user_id: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db::user::update_user_details_v2(&app_state.pool, *user_id, item.into_inner()).await?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(OK_JSON))
}

pub async fn update_user_details_v2b(
    app_state: web::Data<AppState>,
    query_user: web::Query<QueryUser>,
    item: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    db::user::update_user_details_v2(&app_state.pool, query_user.id, item.into_inner()).await?;

    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(OK_JSON))
}
