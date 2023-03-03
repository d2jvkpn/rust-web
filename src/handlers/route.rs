use crate::middlewares::{health_check, health_check_v1, health_check_v2};
use actix_web::web::{get, post, scope, ServiceConfig};

fn open(cfg: &mut ServiceConfig) {
    use super::user_open::*;

    cfg.route("/healthz", get().to(health_check))
        .route("healthz_v1", get().to(health_check_v1))
        .route("/healthz_v2", get().to(health_check_v2));

    let open = scope("/api/open")
        .route("/user/register", post().to(post_new_user))
        .route("/user/login", post().to(user_login));

    cfg.service(open);
}

pub fn auth(cfg: &mut ServiceConfig) {
    use super::user_auth::*;

    let auth = scope("/api/auth")
        .route("/user/update/{user_id}", post().to(update_user_details))
        .route("/user/update_v2a/{user_id}", post().to(update_user_details_v2a))
        .route("/user/update_v2b", post().to(update_user_details_v2b))
        .route("/user/query", get().to(query_users))
        .route("/user/find", get().to(find_user))
        .route("/user/update_status", get().to(update_user_status));

    cfg.service(auth);
}

pub fn route(cfg: &mut ServiceConfig) {
    open(cfg);
}
