use crate::{
    db::db_user,
    internal::AppState,
    middlewares::{Data, IntoResult},
    models::{self, token::Platform, user::*},
    utils,
};
use actix_web::{
    error::Error as ActixError, http::header::HeaderName, web, HttpRequest, HttpResponse,
};
use serde_json::json;
use std::str::FromStr;

pub async fn password(mut request: HttpRequest) -> Result<HttpResponse, ActixError> {
    Data(json!({"chars": models::PASSWORD_CHARS, "range": models::PASSWORD_RANGE}))
        .into_result(&mut request)
}

pub async fn version(mut request: HttpRequest) -> Result<HttpResponse, ActixError> {
    // Data(utils::GIT_BUILD_INFO.clone()).into_result(&mut request)
    Data(utils::GitBuildInfo::get()).into_result(&mut request)
}

pub async fn post_new_user(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    item: web::Json<CreateUser>,
) -> Result<HttpResponse, ActixError> {
    db_user::post_new_user(&app_state.pool, item.into_inner()).await.into_result(&mut request)
}

pub async fn user_login(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    item: web::Json<UserLogin>,
) -> Result<HttpResponse, ActixError> {
    let platform = extract_platform_v1(&request);

    db_user::user_login(&app_state.pool, item.into_inner(), request.peer_addr(), platform)
        .await
        .into_result(&mut request)
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

pub async fn refresh_token(
    mut request: HttpRequest,
    app_state: web::Data<AppState>,
    item: web::Json<RefreshToken>,
) -> Result<HttpResponse, ActixError> {
    let platform = extract_platform_v1(&request);

    db_user::refresh_token(&app_state.pool, item.into_inner(), request.peer_addr(), platform)
        .await
        .into_result(&mut request)
}
