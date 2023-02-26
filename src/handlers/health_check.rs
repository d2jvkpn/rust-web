use crate::internal::AppState;
use actix_web::{
    web::{Data, ReqData},
    HttpResponse,
};
use serde_json::json;
use uuid::Uuid;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn health_check_v1(request_id: ReqData<Uuid>) -> HttpResponse {
    HttpResponse::Ok().json(json!({"code": 0, "msg": "ok", "requestId": request_id.into_inner()}))
}

pub async fn health_check_v2(app_state: Data<AppState>, request_id: ReqData<Uuid>) -> HttpResponse {
    let request_id = request_id.into_inner();
    let response_msg = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let msg = format!("{} {} times", response_msg, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(json!({"code": 0, "msg": msg, "requestId": request_id}))
}
