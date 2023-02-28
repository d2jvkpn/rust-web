use crate::{
    db,
    handlers::response::{Data, Error},
    internal::AppState,
    models::user::CreateUser,
};
use actix_web::{web, HttpResponse};

pub async fn post_new_user(
    app_state: web::Data<AppState>,
    item: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    db::user::post_new_user(&app_state.pool, item.into_inner()).await.map(|v| Ok(Data(v).into()))?
}
