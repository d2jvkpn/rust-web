use crate::{
    internal::{auth_role, settings::Settings},
    middlewares::{blocker::Blocker, health_check},
    models::user::Role,
};
use actix_web::web::{get, post, scope, ServiceConfig};

fn open(cfg: &mut ServiceConfig) {
    use super::open::*;

    cfg.route("/healthz", get().to(health_check));

    let group_open = scope("/api/open")
        .route("/version", get().to(version))
        .route("/password", get().to(password))
        .route("/user/register", post().to(post_new_user))
        .route("/user/login", post().to(user_login))
        .route("/user/refresh_token", post().to(refresh_token));

    cfg.service(group_open);
}

pub fn auth_user(cfg: &mut ServiceConfig) {
    use super::auth_user::*;

    let group = scope("/api/auth/user")
        .wrap(Blocker { block: |req| Ok(Settings::jwt_verify_request(req)?) })
        .route("/update", post().to(update_user_details_v3))
        .route("/details", get().to(user_details))
        .route("/frozon", post().to(frozen_user_status))
        .route("/change_password", post().to(user_change_password))
        .route("/logout", post().to(user_logout));

    cfg.service(group);
}

pub fn auth_leader(cfg: &mut ServiceConfig) {
    // use super::auth_leader::*;

    let group = scope("/api/auth/leader").wrap(auth_role::Auth { value: Role::Leader });

    cfg.service(group);
}

pub fn auth_admin(cfg: &mut ServiceConfig) {
    use super::auth_admin::*;

    let group_user = scope("/api/auth/admin/user")
        .wrap(auth_role::Auth { value: Role::Admin })
        .route("/query", post().to(query_users))
        .route("/find", get().to(find_user))
        .route("/update_status", post().to(update_user_status))
        .route("/update_role", post().to(update_user_role))
        .route("/reset_password", post().to(reset_user_password));

    cfg.service(group_user);
}

pub fn route(cfg: &mut ServiceConfig) {
    open(cfg);
    auth_user(cfg);
    auth_leader(cfg);
    auth_admin(cfg);
}
