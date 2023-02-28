use super::{health_check::*, user::*};
use actix_web::web::{get, post, scope, ServiceConfig};

fn open(cfg: &mut ServiceConfig) {
    let open = scope("/open")
        .route("/user/register", post().to(post_new_user))
        .route("/user/update/{user_id}", post().to(update_user_details))
        .route("/user/update_v2/{user_id}", post().to(update_user_details_v2));

    cfg.route("/healthz", get().to(health_check))
        .route("healthz_v1", get().to(health_check_v1))
        .route("/healthz_v2", get().to(health_check_v2));

    cfg.service(open);
}

pub fn route(cfg: &mut ServiceConfig) {
    open(cfg);
}
