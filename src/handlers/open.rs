use crate::{
    db::user as db_user,
    internal::AppState,
    middlewares::response::{Data, Error},
    models::user::*,
};
use actix_web::{web, HttpResponse};

pub async fn post_new_user(
    app_state: web::Data<AppState>,
    item: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    db_user::post_new_user(&app_state.pool, item.into_inner()).await.map(|v| Ok(Data(v).into()))?
}

pub async fn user_login(
    app_state: web::Data<AppState>,
    login: web::Json<UserLogin>,
) -> Result<HttpResponse, Error> {
    db_user::user_login(&app_state.pool, login.into_inner()).await.map(|v| Ok(Data(v).into()))?
}
