use crate::{
    db::db_user,
    internal::AppState,
    middlewares::response::{Data, Error},
    models::{token::Platform, user::*},
};
use actix_web::{http::header::HeaderName, web, HttpRequest, HttpResponse};
use std::str::FromStr;

pub async fn post_new_user(
    app_state: web::Data<AppState>,
    item: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    db_user::post_new_user(&app_state.pool, item.into_inner()).await.map(|v| Ok(Data(v).into()))?
}

pub async fn user_login(
    app_state: web::Data<AppState>,
    login: web::Json<UserLogin>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let platform = extract_platform_v1(&request);

    db_user::user_login(&app_state.pool, login.into_inner(), request.peer_addr(), platform)
        .await
        .map(|v| Ok(Data(v).into()))?
}

fn extract_platform_v1(request: &HttpRequest) -> Platform {
    let header = HeaderName::from_static("x-platform");

    let value = match request.headers().get(header) {
        Some(v) => v,
        None => return Platform::Unknown,
    };

    match value.to_str() {
        Ok(v) => Platform::from_str(v).unwrap_or(Platform::Unknown),
        Err(_) => Platform::Unknown,
    }
}

#[allow(dead_code)]
fn extract_platform_v2(request: &HttpRequest) -> Option<Platform> {
    let header = HeaderName::from_static("x-platform");

    let value = request.headers().get(header)?;
    let val_str = value.to_str().ok()?;
    Platform::from_str(val_str).ok()
}
