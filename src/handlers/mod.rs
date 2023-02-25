use crate::internal::AppState;
use actix_web::{
    web::{get, Data, ReqData, ServiceConfig},
    HttpResponse,
};
use serde_json::json;
use uuid::Uuid;

async fn health_check(request_id: ReqData<Uuid>) -> HttpResponse {
    HttpResponse::Ok().json(json!({"code": 0, "msg": "ok", "requestId": rquest_id.into_inner()}))
}

async fn health_check_v2(app_state: Data<AppState>, request_id: ReqData<Uuid>) -> HttpResponse {
    let request_id = request_id.into_inner();
    let response_msg = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let msg = format!("{} {} times", response_msg, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(json!({"code": 0, "msg": msg, "requestId": request_id}))
}

fn open(cfg: &mut ServiceConfig) {
    cfg.route("/healthz", get().to(health_check)).route("/healthz_v2", get().to(health_check_v2));
}

pub fn route(cfg: &mut ServiceConfig) {
    open(cfg);
}
