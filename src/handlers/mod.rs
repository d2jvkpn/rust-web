use crate::internal::data::AppState;
use actix_web::{
    web::{self, get, ServiceConfig},
    HttpResponse,
};
use serde_json::json;

async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    let response_msg = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let msg = format!("{} {} times", response_msg, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(json!({"code":0,"msg":msg}))
}

fn open(cfg: &mut ServiceConfig) {
    cfg.route("/healthz", get().to(health_check));
}

pub fn route(cfg: &mut ServiceConfig) {
    open(cfg);
}
