use super::health_check::*;
use actix_web::web::{get, ServiceConfig};

fn open(cfg: &mut ServiceConfig) {
    cfg.route("/healthz", get().to(health_check))
        .route("healthz_v1", get().to(health_check_v1))
        .route("/healthz_v2", get().to(health_check_v2));
}

pub fn route(cfg: &mut ServiceConfig) {
    open(cfg);
}
