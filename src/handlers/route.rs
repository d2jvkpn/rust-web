use crate::{
    internal::{jwt_role, settings::Config},
    middlewares::{blocker::Blocker, health_check},
    models::user::Role,
};
use actix_web::web::{get, post, scope, ServiceConfig};

fn open(cfg: &mut ServiceConfig) {
    use super::open::*;

    cfg.route("/healthz", get().to(health_check));

    let open = scope("/api/open")
        .route("/user/register", post().to(post_new_user))
        .route("/user/login", post().to(user_login));

    cfg.service(open);
}

pub fn auth_user(cfg: &mut ServiceConfig) {
    use super::auth_user::*;

    let group = scope("/api/auth")
        .wrap(Blocker { block: |req| Ok(Config::jwt_verify(req)?) })
        .route("/user/update", post().to(update_user_details_v3))
        .route("/user/details", get().to(user_details))
        .route("/user/frozon", post().to(frozen_user_status))
        .route("/user/change_password", post().to(user_change_password));

    cfg.service(group);
}

pub fn auth_leader(cfg: &mut ServiceConfig) {
    // use super::auth_leader::*;

    let group = scope("/api/auth").wrap(jwt_role::Auth { value: Role::Leader });

    cfg.service(group);
}

pub fn auth_admin(cfg: &mut ServiceConfig) {
    use super::auth_admin::*;

    let group = scope("/api/auth")
        .wrap(jwt_role::Auth { value: Role::Admin })
        .route("/admin/user/query", post().to(query_users))
        .route("/admin/user/find", get().to(find_user))
        .route("/admin/user/update_status", post().to(update_user_status))
        .route("/admin/user/update_role", post().to(update_user_role))
        .route("/admin/user/reset_password", post().to(reset_user_password));

    cfg.service(group);
}

pub fn route(cfg: &mut ServiceConfig) {
    open(cfg);
    auth_user(cfg);
    auth_leader(cfg);
    auth_admin(cfg);
}
